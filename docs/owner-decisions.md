# Owner decisions

A dated record of the decisions the owner made for this repository, so the rules do not depend on anyone's memory of a conversation. Changing any entry needs the owner. An agent that finds a decision blocking a ticket stops and says so.

All entries are dated 2026-11-22.

| # | Decision | Date | Implemented in |
|---|---|---|---|
| 1 | The instructions, tooling and playbook live on the default branch (`main`). | 2026-11-22 | `CLAUDE.md`, `.claude/`, `.github/` |
| 2 | Persona agents run automatically during ticket work and carry instructions tailored to this project. | 2026-11-22 | `.claude/agents/` ("Use proactively" sentences, "In this repository" sections), `.claude/agency-agents/README.md`, `CLAUDE.md` "Agents and ticket work" |
| 3 | Every open ticket is about app implementation; tooling is installed, not ticketed. | 2026-11-22 | `.claude/`, `.github/`, `docs/`, `CLAUDE.md` "Delivery workflow" |
| 4 | Agents may take a ticket up to a ready pull request. Only the owner merges and releases. | 2026-11-22 | `CLAUDE.md` "Git workflow" and "Delivery workflow", `.claude/skills/next-ticket/SKILL.md`, `.claude/skills/implement-ticket/SKILL.md`, `docs/RELEASING.md`, `.claude/settings.json` (`gh pr merge` and `gh release create` denied) |
| 5 | No scheduler. Ticket work starts only on the owner's go signal (`/next-ticket`), because the repository is used to train AI models. | 2026-11-22 | `.claude/skills/next-ticket/SKILL.md` (`disable-model-invocation: true`), `LOOP.md` |
| 6 | A review gate hook blocks opening or readying a pull request until the gate roles have run after the last code edit. | 2026-11-22 | `.claude/hooks/role_gate.py`, `.claude/settings.json` (`hooks`), `.gitignore` (`.claude/state/`) |
| 7 | Loop Engineering stays at L1, report-only and unscheduled. | 2026-11-22 | `LOOP.md`, `loop-constraints.md`, `loop-budget.md` |
| 8 | Continuous integration runs on Linux and Windows; the default branch requires a pull request and green checks. | 2026-11-22 | `.github/workflows/ci.yml`; branch protection is a repository setting applied by the owner's orchestrator, not a file |
| 9 | Permission rules: documented commands are allowed; force pushes, pushes to the default branch, merges, releases and reads of secrets are denied; edits to CI, agent configuration and Loop files ask first. | 2026-11-22 | `.claude/settings.json` (`permissions`) |
| 10 | Open decisions on tickets are settled with the ticket's stated default; where a ticket states none, the conservative option is taken and flagged in the issue. | 2026-11-22 | The list below (decisions D1 to D15 of epic #1); the issues themselves |
| 11 | Rust is verified in CI only; nothing is installed on the owner's PC. | 2026-11-22 | `.github/workflows/ci.yml` (job `rust`), `CLAUDE.md` "Setup and commands" |
| 12 | The ten UI rules in `docs/ui-quality.md` are binding, and the `ui-ux-pro-max` and `unslop` skills are installed in the repository and used for UI work and for copy. | 2026-11-22 | `docs/ui-quality.md`, `.claude/skills/ui-ux-pro-max/`, `.claude/skills/unslop/`, `.claude/skills/implement-ticket/SKILL.md` |

## Open ticket decisions settled by default (entry 10)

Source: the epic #1 body and each ticket's "Owner decisions and defaults" text. Each applies with its default unless the owner changes it; where a ticket states no default, the conservative option is taken and flagged in the issue.

| Decision | Subject | Default | Issue | Status |
|---|---|---|---|---|
| D1 | Binary and crate name | `gitcompat` (package `git-compatible-repository-tool`) | #2 | settled by default |
| D2 | Rust toolchain pin | the stable release current when #2 starts, in `rust-toolchain.toml` | #2 | settled by default |
| D3 | Crates | `sha1` and `flate2` only, default Rust backend, vendored and committed | #2 | settled by default |
| D4 | Project licence | none selected; README keeps "not selected"; vendored crate licences recorded in `docs/dependencies.md` | #2 | settled by default |
| D5 | Limits | values in the "Shared definitions" section of epic #1 (heading "Limits"), in one `limits` module | #2 | settled by default |
| D6 | Platforms | CI on `ubuntu-latest` and `windows-latest`; mode-bit and stat-exact tests run on Linux only, skipped with a recorded reason on Windows | #2 to #7 | settled by default |
| D7 | Minimum Git | 2.45 for `--ref-format=files`; older Git omits the flag and the report says so | #2 | settled by default |
| D8 | Output | JSON on stdout is the contract; text via `--format text`; diagnostics on stderr | #2, #7 | settled by default |
| D9 | Identity and time | fixed fixture identity; timestamp `1704067200 +0000` unless a flag overrides it | #2, #5 | settled by default |
| D10 | Fixture ownership | `.gcrt-fixture` marker required for any write | #2 | settled by default |
| D11 | Case-fold collisions | rejected on every platform | #3 | settled by default |
| D12 | Ambiguous merge base | refuse; no recursive merge | #5 | settled by default |
| D13 | Unknown formats | fail closed with a typed code | E44-7 (#8) | settled by default |
| D14 | Compatibility report | checked in once from a CI run | #7 | settled by default |
| D15 | Normalization and duplicate-path collisions | duplicate paths and case-fold collisions fail with `path_collision`; path components may only use an allowlist of NFC-stable code points (ASCII and U+00A0 to U+017F); any other code point fails with `path_unicode_unsupported` (fail closed, no normalization table or extra crate), so NFC/NFD and singleton-decomposition spellings can never both be stored; both checks run before any write, on every platform; widening the allowlist with a table or crate is an owner decision | #3 | settled by default |

Per-ticket defaults beyond D1 to D15: #2: the toolchain pin is the stable release current when the ticket starts; #4: on Windows, stat fields without an equivalent are written as zero and status relies on the rehash, and the exact stat comparison runs on Linux only; #5: the merge commit message is `Merge <branch>` unless `--message` is given and there is no text-level conflict resolution; #6: only undeltified objects, no pack writing; #3: only `refs/heads/**` is writable and branch names follow `git check-ref-format`. Ticket E44-7 (typed diagnostics) is issue #8; its default is D13.

No open decision lacks a default.
