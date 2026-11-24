use std::process::ExitCode;

use yamin_test::cases::{all, Case, Category};
use yamin_test::terminal::{render_case, Opts};

const USAGE: &str = "\
usage: yamin-test <command> [options]

commands:
  list   [--category C]                 list the 40 cases
  show   [--category C] [--id ID]       print cases: actual line + expected line
  html   [--out FILE]                   write the browser reference page

display options (show):
  --no-escapes    do not emit BDSM/SCP escape sequences
  --no-controls   do not wrap the expected line in LRO..PDF
  --codepoints    also print the expected line's code points

categories: arabic mixed digits paths tashkeel lamalef mirroring code";

struct Args {
    cmd: String,
    category: Option<Category>,
    id: Option<String>,
    out: Option<String>,
    opts: Opts,
}

fn parse(argv: &[String]) -> Result<Args, String> {
    let mut it = argv.iter();
    let cmd = it.next().ok_or("missing command")?.clone();
    let mut a = Args { cmd, category: None, id: None, out: None, opts: Opts::default() };
    while let Some(arg) = it.next() {
        let mut value = |name: &str| {
            it.next().cloned().ok_or_else(|| format!("{name} needs a value"))
        };
        match arg.as_str() {
            "--category" => {
                let v = value("--category")?;
                a.category = Some(
                    Category::from_slug(&v).ok_or_else(|| format!("unknown category: {v}"))?,
                );
            }
            "--id" => a.id = Some(value("--id")?),
            "--out" => a.out = Some(value("--out")?),
            "--no-escapes" => a.opts.escapes = false,
            "--no-controls" => a.opts.controls = false,
            "--codepoints" => a.opts.codepoints = true,
            other => return Err(format!("unknown option: {other}")),
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
