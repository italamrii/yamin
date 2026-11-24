//! Interactive evaluation: show each case, let the user judge pass/fail.

use std::io::{self, BufRead, Write};

use serde::{Deserialize, Serialize};

use crate::cases::Case;
use crate::terminal::{render_case, Opts};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Verdict {
    Pass,
    Fail,
    Skip,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CaseResult {
    pub id: String,
    pub category: String,
    pub verdict: Verdict,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Report {
    pub tool: String,
    pub version: String,
    pub terminal: String,
    /// The AI CLI tool the cases were viewed through ("none" for a bare shell).
    pub cli: String,
    pub date: String,
    pub results: Vec<CaseResult>,
}

pub struct Meta {
    pub terminal: String,
    pub cli: String,
    pub date: String,
}

enum Answer {
    Verdict(Verdict),
    Quit,
}

fn parse_answer(line: &str) -> Option<Answer> {
    match line.trim().to_lowercase().as_str() {
        "p" | "pass" => Some(Answer::Verdict(Verdict::Pass)),
        "f" | "fail" => Some(Answer::Verdict(Verdict::Fail)),
        "s" | "skip" => Some(Answer::Verdict(Verdict::Skip)),
        "q" | "quit" => Some(Answer::Quit),
        _ => None,
    }
}

/// Reads one line; `None` on EOF.
fn read_line<R: BufRead>(input: &mut R) -> io::Result<Option<String>> {
    let mut buf = String::new();
    if input.read_line(&mut buf)? == 0 {
        return Ok(None);
    }
    Ok(Some(buf.trim_end_matches(['\r', '\n']).to_string()))
}

pub fn run_eval<R: BufRead, W: Write>(
    cases: &[&Case],
    meta: Meta,
    mut input: R,
    mut out: W,
    opts: Opts,
) -> io::Result<Report> {
    let mut results = Vec::with_capacity(cases.len());
    let mut quit = false;

    for (n, case) in cases.iter().enumerate() {
        let mut result = CaseResult {
            id: case.id.to_string(),
            category: case.category.slug().to_string(),
            verdict: Verdict::Skip,
            note: None,
        };
        if quit {
            result.note = Some("not evaluated".into());
            results.push(result);
            continue;
        }

        writeln!(out, "\n({}/{})", n + 1, cases.len())?;
        write!(out, "{}", render_case(case, opts))?;
        loop {
            write!(out, "pass / fail / skip / quit [p/f/s/q] > ")?;
            out.flush()?;
            let Some(line) = read_line(&mut input)? else {
                quit = true;
                break;
            };
            match parse_answer(&line) {
                Some(Answer::Verdict(v)) => {
                    result.verdict = v;
                    if v == Verdict::Fail {
                        write!(out, "what looks wrong? (optional) > ")?;
                        out.flush()?;
                        let note = read_line(&mut input)?.unwrap_or_default();
                        let note = note.trim();
                        if !note.is_empty() {
                            result.note = Some(note.to_string());
                        }
                    }
                    break;
                }
                Some(Answer::Quit) => {
                    quit = true;
                    break;
                }
                None => writeln!(out, "please answer p, f, s or q")?,
            }
        }
        if quit {
            result.note = Some("not evaluated".into());
        }
        results.push(result);
    }

    Ok(Report {
        tool: "yamin-test".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        terminal: meta.terminal,
        cli: meta.cli,
        date: meta.date,
        results,
    })
}
