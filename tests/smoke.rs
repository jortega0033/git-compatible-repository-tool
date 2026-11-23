//! Scaffold smoke test: the built binary starts and answers `--help` and `--version`.

use std::process::Command;

fn gitcompat(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_gitcompat"))
        .args(args)
        .output()
        .expect("gitcompat starts")
}

#[test]
fn help_prints_usage() {
    let output = gitcompat(&["--help"]);
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("usage: gitcompat"));
}

#[test]
fn version_prints_name() {
    let output = gitcompat(&["--version"]);
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.starts_with("gitcompat "));
}

#[test]
fn other_input_fails_without_stdout() {
    let output = gitcompat(&["init"]);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
}
