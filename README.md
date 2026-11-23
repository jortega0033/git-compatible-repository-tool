# Git-compatible Repository Tool

A planned offline project. Product implementation has not started. The repository holds the product plan, technical specification, implementation backlog and an empty Rust crate scaffold.

## Plan

- [Full product plan and acceptance criteria](docs/PRD.md)
- [Technical specification and verification fixtures](docs/TECHNICAL_SPEC.md)
- [Implementation epic](https://github.com/jortega0033/git-compatible-repository-tool/issues/1)

## Current status

The fixtures describe expected results for future checks. The scaffold builds as `gitcompat` (`cargo build --locked --offline`, `cargo test --locked --offline`) and answers `gitcompat --help` and `--version`; the tool's commands exit 1 until their tickets land. No passing implementation tests are claimed beyond the smoke tests, and the Rust checks run in CI only.

## Licensing

A license for this project has not been selected. Any upstream reference or adapted code must retain its applicable notices; see the plan.
