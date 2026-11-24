use std::process::ExitCode;

use yamin_test::cases::{all, Case, Category};
use yamin_test::eval::{run_eval, Meta};
use yamin_test::report;
use yamin_test::terminal::{render_case, Opts};

const USAGE: &str = "\
usage: yamin-test <command> [options]

commands:
  list   [--category C]                 list the 40 cases
  show   [--category C] [--id ID]       print cases: actual line + expected line
  html   [--out FILE]                   write the browser reference page
  eval   --terminal T --cli C [--out DIR] [--category C]
                                        judge each case pass/fail; writes
                                        DIR/<stem>.json and DIR/<stem>.md
                                        (--cli none = bare shell; DIR defaults to reports)
  matrix [--reports DIR] [--out FILE]   terminals x AI CLI table from DIR/*.json

display options (show, eval):
  --no-escapes    do not emit BDSM/SCP escape sequences
  --no-controls   do not wrap the expected line in LRO..PDF
  --codepoints    also print the expected line's code points
  --width N       terminal width for right-aligning RTL lines
                  (default: detected terminal width, else 48)

categories: arabic mixed digits paths tashkeel lamalef mirroring code";

struct Args {
    cmd: String,
    category: Option<Category>,
    id: Option<String>,
    out: Option<String>,
    terminal: Option<String>,
    cli: Option<String>,
    reports: Option<String>,
    opts: Opts,
}

fn parse(argv: &[String]) -> Result<Args, String> {
    let mut it = argv.iter();
    let cmd = it.next().ok_or("missing command")?.clone();
    let mut a = Args {
        cmd,
        category: None,
        id: None,
        out: None,
        terminal: None,
        cli: None,
        reports: None,
        opts: Opts::default(),
    };
    let mut width_given = false;
    while let Some(arg) = it.next() {
        let mut value = |name: &str| {
            it.next()
                .cloned()
                .ok_or_else(|| format!("{name} needs a value"))
        };
        match arg.as_str() {
            "--category" => {
                let v = value("--category")?;
                a.category =
                    Some(Category::from_slug(&v).ok_or_else(|| format!("unknown category: {v}"))?);
            }
            "--id" => a.id = Some(value("--id")?),
            "--out" => a.out = Some(value("--out")?),
            "--terminal" => a.terminal = Some(value("--terminal")?),
            "--cli" => a.cli = Some(value("--cli")?),
            "--reports" => a.reports = Some(value("--reports")?),
            "--no-escapes" => a.opts.escapes = false,
            "--no-controls" => a.opts.controls = false,
            "--codepoints" => a.opts.codepoints = true,
            "--width" => {
                let v = value("--width")?;
                a.opts.width = v
                    .parse()
                    .map_err(|_| format!("--width needs a number of columns, got {v}"))?;
                width_given = true;
            }
            other => return Err(format!("unknown option: {other}")),
        }
    }
    if !width_given {
        // Right-align to the real terminal; padding past it would wrap the line.
        if let Some((terminal_size::Width(w), _)) = terminal_size::terminal_size() {
            a.opts.width = w as usize;
        }
    }
    Ok(a)
}

fn selected(a: &Args) -> Result<Vec<&'static Case>, String> {
    let cases: Vec<&Case> = all()
        .iter()
        .filter(|c| a.category.is_none_or(|cat| c.category == cat))
        .filter(|c| a.id.as_deref().is_none_or(|id| c.id == id))
        .collect();
    if cases.is_empty() {
        return Err("no case matches".into());
    }
    Ok(cases)
}

fn run(a: Args) -> Result<(), String> {
    match a.cmd.as_str() {
        "list" => {
            for c in selected(&a)? {
                println!("{:<14} {:<4} {}", c.id, c.dir.as_str(), c.input);
            }
        }
        "show" => {
            for c in selected(&a)? {
                println!("{}", render_case(c, a.opts));
            }
        }
        "html" => {
            let page = yamin_test::html::reference_page();
            let path = a.out.unwrap_or_else(|| "yamin-reference.html".into());
            std::fs::write(&path, page).map_err(|e| format!("{path}: {e}"))?;
            println!("wrote {path}");
        }
        "eval" => {
            let terminal = a.terminal.clone().ok_or("eval needs --terminal NAME")?;
            let cli = a
                .cli
                .clone()
                .ok_or("eval needs --cli NAME (use \"none\" for a bare shell)")?;
            let cases = selected(&a)?;
            let meta = Meta {
                terminal,
                cli,
                date: report::today(),
            };
            let stdin = std::io::stdin();
            let r = run_eval(&cases, meta, stdin.lock(), std::io::stdout(), a.opts)
                .map_err(|e| e.to_string())?;
            let dir = std::path::PathBuf::from(a.out.as_deref().unwrap_or("reports"));
            std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
            let stem = report::file_stem(&r);
            for (ext, body) in [
                ("json", report::to_json(&r)),
                ("md", report::to_markdown(&r)),
            ] {
                let path = dir.join(format!("{stem}.{ext}"));
                std::fs::write(&path, body).map_err(|e| format!("{}: {e}", path.display()))?;
                println!("wrote {}", path.display());
            }
            let s = report::summary(&r);
            println!(
                "{}/{} passed ({}%), {} skipped",
                s.pass,
                s.evaluated(),
                s.pass_pct(),
                s.skip
            );
        }
        "matrix" => {
            let dir = a.reports.as_deref().unwrap_or("reports");
            let md = yamin_test::matrix::build(&yamin_test::matrix::load_dir(dir.as_ref())?);
            match a.out {
                Some(path) => {
                    std::fs::write(&path, md).map_err(|e| format!("{path}: {e}"))?;
                    println!("wrote {path}");
                }
                None => print!("{md}"),
            }
        }
        "help" | "--help" | "-h" => println!("{USAGE}"),
        other => return Err(format!("unknown command: {other}")),
    }
    Ok(())
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
