use std::io::Cursor;

use yamin_test::cases::{all, Case};
use yamin_test::eval::{run_eval, Meta, Verdict};
use yamin_test::report::{civil_date, file_stem, from_json, summary, to_json, to_markdown};
use yamin_test::terminal::Opts;

fn meta() -> Meta {
    Meta {
        terminal: "Windows Terminal".into(),
        cli: "Claude Code".into(),
        date: "2026-11-24".into(),
    }
}

fn first(n: usize) -> Vec<&'static Case> {
    all().iter().take(n).collect()
}

fn eval(cases: &[&Case], script: &str) -> yamin_test::eval::Report {
    let mut out = Vec::new();
    run_eval(
        cases,
        meta(),
        Cursor::new(script),
        &mut out,
        Opts::default(),
    )
    .unwrap()
}

#[test]
fn records_pass_fail_skip_and_notes() {
    let r = eval(&first(3), "p\nf\nlam-alef split\ns\n");
    let v: Vec<Verdict> = r.results.iter().map(|x| x.verdict).collect();
    assert_eq!(v, vec![Verdict::Pass, Verdict::Fail, Verdict::Skip]);
    assert_eq!(r.results[1].note.as_deref(), Some("lam-alef split"));
    assert_eq!(r.results[0].note, None);
    assert_eq!(r.terminal, "Windows Terminal");
    assert_eq!(r.cli, "Claude Code");
}

#[test]
fn accepts_full_words_and_reprompts_on_garbage() {
    let r = eval(&first(2), "maybe\nPASS\nfail\n\n");
    assert_eq!(r.results[0].verdict, Verdict::Pass);
    assert_eq!(r.results[1].verdict, Verdict::Fail);
    assert_eq!(r.results[1].note, None, "empty note is None");
}

#[test]
fn quit_marks_the_rest_as_skipped() {
    let r = eval(&first(4), "p\nq\n");
    assert_eq!(r.results.len(), 4);
    assert_eq!(r.results[0].verdict, Verdict::Pass);
    assert!(r.results[1..].iter().all(|x| x.verdict == Verdict::Skip));
    assert_eq!(r.results[3].note.as_deref(), Some("not evaluated"));
}

#[test]
fn eof_behaves_like_quit() {
    let r = eval(&first(2), "p\n");
    assert_eq!(r.results[1].verdict, Verdict::Skip);
}

#[test]
fn summary_counts() {
    let r = eval(&first(4), "p\nf\n\np\ns\n");
    let s = summary(&r);
    assert_eq!((s.pass, s.fail, s.skip, s.total), (2, 1, 1, 4));
    assert_eq!(s.evaluated(), 3);
}

#[test]
fn json_round_trips() {
    let r = eval(&first(2), "p\nf\nbad\n");
    let json = to_json(&r);
    assert!(json.contains("\"verdict\": \"fail\""));
    assert_eq!(from_json(&json).unwrap(), r);
}

#[test]
fn markdown_has_summary_categories_and_failures() {
    let r = eval(&first(6), "p\np\np\np\np\nf\n! on wrong side\n");
    let md = to_markdown(&r);
    assert!(md.contains("# yamin-test report"));
    assert!(md.contains("Windows Terminal"));
    assert!(md.contains("| arabic | 5 | 0 | 0 |"));
    assert!(md.contains("| mixed | 0 | 1 | 0 |"));
    assert!(md.contains("mixed-01"));
    assert!(md.contains("! on wrong side"));
}

#[test]
fn file_stem_is_slugged() {
    let r = eval(&first(1), "p\n");
    assert_eq!(
        file_stem(&r),
        "yamin-windows-terminal-claude-code-2026-11-24"
    );
}

#[test]
fn civil_date_from_unix_days() {
    assert_eq!(civil_date(0), "1970-01-01");
    assert_eq!(civil_date(19_723), "2024-01-01");
    assert_eq!(civil_date(20_781), "2026-11-24");
}
