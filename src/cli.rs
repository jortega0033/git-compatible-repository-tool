//! `cli`: argument parsing, command dispatch, exit codes.

use std::process::ExitCode;

/// One-line responsibility, copied from docs/architecture.md.
pub const RESPONSIBILITY: &str = "argument parsing, command dispatch, exit codes.";

/// Commands of ticket #2 and the child ticket that implements each.
const COMMANDS: [(&str, u32); 4] = [
    ("init", 12),
    ("snapshot", 15),
    ("inspect", 16),
    ("preflight", 17),
];

const USAGE: &str = "usage: gitcompat <command> --root <fixture-root>\n\
commands: init, snapshot, inspect, preflight\n\
options: --help, --version";

/// What a run produced: exit code and the text for each stream.
#[derive(Debug, PartialEq, Eq)]
pub struct Outcome {
    pub code: u8,
    pub stdout: String,
    pub stderr: String,
}

fn outcome(code: u8, stdout: String, stderr: String) -> Outcome {
    Outcome { code, stdout, stderr }
}

fn version_line() -> String {
    format!("gitcompat {}\n", env!("CARGO_PKG_VERSION"))
}

/// Decide the outcome for the given arguments (without the program name).
pub fn execute(args: &[String]) -> Outcome {
    match args.first().map(String::as_str) {
        Some("--help" | "-h") => outcome(0, format!("{USAGE}\n"), String::new()),
        Some("--version" | "-V") => outcome(0, version_line(), String::new()),
        Some(name) => match COMMANDS.iter().find(|(command, _)| *command == name) {
            Some((command, ticket)) => {
                let message = format!("not implemented yet: {command}, see #{ticket}\n");
                outcome(1, String::new(), message)
            }
            None => outcome(2, String::new(), format!("unknown command: {name}\n{USAGE}\n")),
        },
        None => outcome(2, String::new(), format!("{USAGE}\n")),
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

    #[test]
    fn planned_command_names_its_ticket() {
        let result = execute(&args(&["snapshot"]));
        assert_eq!(result.code, 1);
        assert!(result.stdout.is_empty());
        assert!(result.stderr.contains("not implemented yet"));
        assert!(result.stderr.contains("see #15"));
    }

    #[test]
    fn unknown_command_is_a_usage_error() {
        let result = execute(&args(&["bogus"]));
        assert_eq!(result.code, 2);
        assert!(result.stdout.is_empty());
    }
}
