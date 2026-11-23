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
fn unbuilt_command_fails_with_ticket() {
    let output = gitcompat(&["init"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let text = String::from_utf8_lossy(&output.stderr);
    assert!(text.contains("see #12"));
}
