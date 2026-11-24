use yamin_test::eval::{CaseResult, Report, Verdict};
use yamin_test::matrix::build;

fn report(terminal: &str, cli: &str, date: &str, pass: usize, fail: usize, skip: usize) -> Report {
    let mut results = Vec::new();
    for (v, n) in [(Verdict::Pass, pass), (Verdict::Fail, fail), (Verdict::Skip, skip)] {
        for _ in 0..n {
            results.push(CaseResult {
                id: "x".into(),
                category: "arabic".into(),
                verdict: v,
                note: None,
            });
        }
    }
    Report {
        tool: "yamin-test".into(),
        version: "0.1.0".into(),
        terminal: terminal.into(),
        cli: cli.into(),
        date: date.into(),
        results,
    }
}

#[test]
fn rows_are_terminals_columns_are_clis() {
    let md = build(&[
        report("WezTerm", "none", "2026-11-24", 30, 10, 0),
        report("Konsole", "Claude Code", "2026-11-24", 40, 0, 0),
    ]);
    let lines: Vec<&str> = md.lines().collect();
    let header = lines.iter().find(|l| l.starts_with("| Terminal")).unwrap();
    assert_eq!(*header, "| Terminal | none | Claude Code |");
    assert!(md.contains("| Konsole | — | 40/40 (100%) |"));
    assert!(md.contains("| WezTerm | 30/40 (75%) | — |"));
}

#[test]
fn denominator_excludes_skipped_cases_and_shows_them() {
    let md = build(&[report("foot", "none", "2026-11-24", 18, 2, 20)]);
    assert!(md.contains("| foot | 18/20 (90%) +20 skipped |"), "{md}");
}

#[test]
fn latest_report_wins_for_the_same_pair() {
    let md = build(&[
        report("foot", "none", "2026-11-25", 40, 0, 0),
        report("foot", "none", "2026-11-01", 1, 39, 0),
    ]);
    assert!(md.contains("40/40"));
    assert!(!md.contains("1/40"));
}

#[test]
fn empty_input_says_so() {
    assert!(build(&[]).contains("No reports"));
}
