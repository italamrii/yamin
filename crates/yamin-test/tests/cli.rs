use std::process::Command;

fn run(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_yamin-test"))
        .args(args)
        .output()
        .expect("run yamin-test");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn list_prints_all_40_ids() {
    let (ok, out, _) = run(&["list"]);
    assert!(ok);
    assert_eq!(out.lines().filter(|l| l.contains("-0")).count(), 40);
}

#[test]
fn list_filters_by_category() {
    let (ok, out, _) = run(&["list", "--category", "paths"]);
    assert!(ok);
    assert_eq!(out.lines().filter(|l| l.starts_with("paths-")).count(), 5);
    assert!(!out.contains("arabic-01"));
}

#[test]
fn show_single_case() {
    let (ok, out, _) = run(&["show", "--id", "lamalef-01", "--no-escapes"]);
    assert!(ok);
    assert!(out.contains("[lamalef-01]"));
    assert!(!out.contains("[lamalef-02]"));
    assert!(!out.contains('\x1b'));
}

#[test]
fn unknown_category_fails() {
    let (ok, _, err) = run(&["list", "--category", "nope"]);
    assert!(!ok);
    assert!(err.contains("unknown category"));
}

#[test]
fn unknown_command_fails_with_usage() {
    let (ok, _, err) = run(&["frobnicate"]);
    assert!(!ok);
    assert!(err.contains("usage"));
}
