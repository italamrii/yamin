use std::io::Write;
use std::process::{Command, Stdio};

fn run(args: &[&str], stdin: &str) -> (bool, String, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_yamin-cat"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("run yamin-cat");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn reads_stdin_line_by_line() {
    let (ok, out, err) = run(&["--width", "20"], "hello\nمرحبا world!\n");
    assert!(ok, "{err}");
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines[0], "hello");
    assert_eq!(
        lines[1],
        format!(
            "{}!world \u{FE8E}\u{FE92}\u{FEA3}\u{FEAE}\u{FEE3}",
            " ".repeat(8)
        )
    );
    assert_eq!(lines.len(), 2);
}

#[test]
fn reads_files_given_as_arguments() {
    let path = std::env::temp_dir().join(format!("yamin-cat-{}.txt", std::process::id()));
    std::fs::write(&path, "لا\n").unwrap();
    let (ok, out, err) = run(&["--no-align", path.to_str().unwrap()], "");
    let _ = std::fs::remove_file(&path);
    assert!(ok, "{err}");
    assert_eq!(out.trim_end_matches(['\r', '\n']), " \u{FEFB}");
}

#[test]
fn forced_direction_and_wrapping() {
    let (ok, out, _) = run(&["--dir", "rtl", "--width", "6"], "مرحبا بالعالم\n");
    assert!(ok);
    assert_eq!(out.lines().count(), 3);
}

#[test]
fn colors_are_stripped() {
    let (ok, out, _) = run(&["--no-align"], "\x1b[31mred\x1b[0m\n");
    assert!(ok);
    assert_eq!(out.trim_end(), "red");
}

#[test]
fn bad_direction_is_an_error() {
    let (ok, _, err) = run(&["--dir", "up"], "");
    assert!(!ok);
    assert!(err.contains("--dir"));
}

#[test]
fn missing_file_is_an_error() {
    let (ok, _, err) = run(&["no-such-file.txt"], "");
    assert!(!ok);
    assert!(err.contains("no-such-file.txt"));
}

#[test]
fn help_prints_usage() {
    let (ok, out, _) = run(&["--help"], "");
    assert!(ok);
    assert!(out.contains("usage: yamin-cat"));
}

#[test]
fn compact_lamalef_flag() {
    let (ok, out, err) = run(&["--no-align", "--compact-lamalef"], "سلام\n");
    assert!(ok, "{err}");
    assert_eq!(out.trim_end(), "\u{FEE1}\u{FEFC}\u{FEB3}");
}
