# Git-compatible Repository Tool

Build an independent Git in Rust, from the on-disk formats up, that a developer could use in place of `git` on a real repository. It interoperates with installed Git in both directions, survives a kill at any point without losing data, and is measured against Git's own test suite. No libgit2, no gitoxide, no calls to Git: installed Git is only the judge.

A planned offline project. Product implementation has not started. The repository holds the product plan, technical specification, implementation backlog and an empty Rust crate scaffold.

## Plan

- [Full product plan and acceptance criteria](docs/PRD.md)
- [Technical specification and verification fixtures](docs/TECHNICAL_SPEC.md)
- [Program: goal, score and milestones](https://github.com/jortega0033/git-compatible-repository-tool/issues/35)
- [Milestone 1 epic](https://github.com/jortega0033/git-compatible-repository-tool/issues/1)

### Score

The share of a pinned, unmodified release of Git's own test suite that passes with this binary in place of `git`. The suite is fetched at a pinned release and run from outside the repository; none of it is copied in. How much of it runs without Git's own helper programs is not known yet: M6 pins the release, lists the usable scripts and publishes that list with the score.

### Milestones

| Milestone | Content | Hours (rough) | Issue |
|---|---|---|---|
| M1 | Objects, refs, index, simple merge | 34 to 53 | [#1](https://github.com/jortega0033/git-compatible-repository-tool/issues/1) |
| M2 | Packs at scale | 60 to 100 | [#36](https://github.com/jortega0033/git-compatible-repository-tool/issues/36) |
| M3 | Everyday commands | 150 to 250 | [#37](https://github.com/jortega0033/git-compatible-repository-tool/issues/37) |
| M4 | Transport | 120 to 200 | [#38](https://github.com/jortega0033/git-compatible-repository-tool/issues/38) |
| M5 | Full repository formats | 100 to 180 | [#39](https://github.com/jortega0033/git-compatible-repository-tool/issues/39) |
| M6 | Proof | 120 to 200 | [#40](https://github.com/jortega0033/git-compatible-repository-tool/issues/40) |

About 580 to 980 engineer hours in total: rough estimates, not measurements. Only M1 is ticketed. M2 to M6 are planned, and no single session is expected to reach the end.

## Current status

The fixtures describe expected results for future checks. The scaffold builds as `gitcompat` (`cargo build --locked --offline`, `cargo test --locked --offline`) and answers `gitcompat --help` and `--version`; any other input exits 1 with "not implemented yet: see #11" until the tickets land. No passing implementation tests are claimed beyond the smoke tests, and the Rust checks run in CI only.

## Licensing

A license for this project has not been selected. Any upstream reference or adapted code must retain its applicable notices; see the plan.
