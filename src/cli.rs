//! `cli`: argument parsing, command dispatch, exit codes.

use std::process::ExitCode;

/// One-line responsibility, copied from docs/architecture.md.
pub const RESPONSIBILITY: &str = "argument parsing, command dispatch, exit codes.";

const USAGE: &str = "usage: gitcompat [--help | --version]";

/// Every other input: the product is not built yet (scaffold).
const NOT_IMPLEMENTED: &str = "not implemented yet: see #11\n";

/// What a run produced: exit code and the text for each stream.
#[derive(Debug, PartialEq, Eq)]
pub struct Outcome {
    pub code: u8,
    pub stdout: String,
    pub stderr: String,
}

fn outcome(code: u8, stdout: String, stderr: String) -> Outcome {
    Outcome {
        code,
        stdout,
        stderr,
    }
}

fn version_line() -> String {
    format!("gitcompat {}\n", env!("CARGO_PKG_VERSION"))
}

/// Decide the outcome for the given arguments (without the program name).
pub fn execute(args: &[String]) -> Outcome {
    match args.first().map(String::as_str) {
        Some("--help" | "-h") => outcome(0, format!("{USAGE}\n"), String::new()),
        Some("--version" | "-V") => outcome(0, version_line(), String::new()),
        _ => outcome(1, String::new(), NOT_IMPLEMENTED.to_string()),
    }
}

/// Run the tool: print the outcome and return its exit code.
pub fn run(args: &[String]) -> ExitCode {
    let result = execute(args);
    print!("{}", result.stdout);
    eprint!("{}", result.stderr);
    ExitCode::from(result.code)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|item| (*item).to_string()).collect()
    }

    #[test]
    fn module_links() {
        assert!(!RESPONSIBILITY.is_empty());
    }

    #[test]
    fn help_and_version_succeed() {
        let help = execute(&args(&["--help"]));
        assert_eq!(help.code, 0);
        let version = execute(&args(&["--version"]));
        assert!(version.stdout.starts_with("gitcompat "));
    }
}
