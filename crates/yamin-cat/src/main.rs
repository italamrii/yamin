use std::io::{self, BufRead, BufWriter, Write};
use std::process::ExitCode;

use yamin_cat::{render, DirMode, Opts};

const USAGE: &str = "\
usage: yamin-cat [options] [FILE...]

Print text with Arabic (and any RTL script) already reordered and shaped, so
it reads correctly in terminals without BiDi support. Reads stdin when no
FILE is given. Each line is one paragraph.

options:
  --dir ltr|rtl|auto   paragraph direction (default: auto, first strong char)
  --width N            wrap/align width (default: terminal width, else 80)
  --no-align           do not right-align RTL lines
  --protect            wrap each line in LRO..PDF (for terminals that do BiDi)
  --compact-lamalef    draw lam-alef in one cell (no gap in non-BiDi terminals)
  --no-color           remove colors instead of keeping them
  -h, --help           show this help

Colors (SGR) are kept and follow their characters after reordering; other
escape sequences (cursor movement, OSC links) are removed. --no-color removes
colors too.";

struct Args {
    opts: Opts,
    files: Vec<String>,
    help: bool,
}

fn parse(argv: &[String]) -> Result<Args, String> {
    let mut a = Args {
        opts: Opts::default(),
        files: Vec::new(),
        help: false,
    };
    let mut width_given = false;
    let mut it = argv.iter();
    while let Some(arg) = it.next() {
        let mut value = |name: &str| {
            it.next()
                .cloned()
                .ok_or_else(|| format!("{name} needs a value"))
        };
        match arg.as_str() {
            "--dir" => {
                a.opts.dir = match value("--dir")?.as_str() {
                    "ltr" => DirMode::Ltr,
                    "rtl" => DirMode::Rtl,
                    "auto" => DirMode::Auto,
                    other => return Err(format!("--dir must be ltr, rtl or auto, got {other}")),
                }
            }
            "--width" => {
                let v = value("--width")?;
                a.opts.width = v
                    .parse()
                    .map_err(|_| format!("--width needs a number, got {v}"))?;
                width_given = true;
            }
            "--no-align" => a.opts.align = false,
            "--protect" => a.opts.protect = true,
            "--compact-lamalef" => a.opts.compact_lamalef = true,
            "--no-color" => a.opts.color = false,
            "-h" | "--help" => a.help = true,
            s if s.starts_with('-') && s != "-" => return Err(format!("unknown option: {s}")),
            file => a.files.push(file.to_string()),
        }
    }
    if !width_given {
        if let Some((terminal_size::Width(w), _)) = terminal_size::terminal_size() {
            a.opts.width = w as usize;
        }
    }
    Ok(a)
}

fn copy<R: BufRead, W: Write>(mut input: R, out: &mut W, opts: &Opts) -> io::Result<()> {
    let mut buf = Vec::new();
    loop {
        buf.clear();
        if input.read_until(b'\n', &mut buf)? == 0 {
            return Ok(());
        }
        let line = String::from_utf8_lossy(&buf);
        let line = line.trim_end_matches(['\n', '\r']);
        for row in render(line, opts) {
            writeln!(out, "{row}")?;
        }
    }
}

fn run(a: Args) -> Result<(), String> {
    if a.help {
        println!("{USAGE}");
        return Ok(());
    }
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    if a.files.is_empty() || a.files == ["-"] {
        copy(io::stdin().lock(), &mut out, &a.opts).map_err(|e| e.to_string())?;
    }
    for path in a.files.iter().filter(|p| *p != "-") {
        let f = std::fs::File::open(path).map_err(|e| format!("{path}: {e}"))?;
        copy(io::BufReader::new(f), &mut out, &a.opts).map_err(|e| format!("{path}: {e}"))?;
    }
    out.flush().map_err(|e| e.to_string())
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    match parse(&argv).and_then(run) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}\n\n{USAGE}");
            ExitCode::FAILURE
        }
    }
}
