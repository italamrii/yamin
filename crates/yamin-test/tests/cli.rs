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

#[test]
fn eval_writes_json_and_markdown_reports() {
    use std::io::Write;
    use std::process::Stdio;

    let dir = std::env::temp_dir().join(format!("yamin-eval-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut child = Command::new(env!("CARGO_BIN_EXE_yamin-test"))
        .args(["eval", "--terminal", "Test Term", "--cli", "none", "--category", "lamalef"])
        .args(["--no-escapes", "--out"])
        .arg(&dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"p\np\nf\nsplit glyph\np\ns\n").unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("3/4 passed (75%), 1 skipped"), "{stdout}");

    let files: Vec<_> = std::fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).collect();
    let json = files.iter().find(|p| p.extension().unwrap() == "json").expect("json");
    let md = files.iter().find(|p| p.extension().unwrap() == "md").expect("md");
    assert!(json.file_name().unwrap().to_str().unwrap().starts_with("yamin-test-term-none-"));
    assert!(std::fs::read_to_string(json).unwrap().contains("split glyph"));
    assert!(std::fs::read_to_string(md).unwrap().contains("lamalef-03"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn eval_requires_terminal_and_cli() {
    let (ok, _, err) = run(&["eval", "--cli", "none"]);
    assert!(!ok);
    assert!(err.contains("--terminal"));
}

#[test]
fn matrix_reads_reports_dir() {
    let dir = std::env::temp_dir().join(format!("yamin-matrix-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("a.json"),
        r#"{"tool":"yamin-test","version":"0.1.0","terminal":"foot","cli":"none","date":"2026-11-24",
            "results":[{"id":"arabic-01","category":"arabic","verdict":"pass"},
                       {"id":"arabic-02","category":"arabic","verdict":"fail","note":"x"}]}"#,
    )
    .unwrap();
    std::fs::write(dir.join("a.md"), "ignored").unwrap();
    let (ok, out, err) = run(&["matrix", "--reports", dir.to_str().unwrap()]);
    assert!(ok, "{err}");
    assert!(out.contains("| foot | 1/2 (50%) |"), "{out}");
    let _ = std::fs::remove_dir_all(&dir);
}
