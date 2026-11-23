# CLAUDE.md

Instructions for work in this repository. Read this before changing anything.

## What this is

A planned, original, offline Rust command-line tool for people learning how Git stores data. It creates a disposable SHA-1 repository from synthetic inputs, stages and merges in a bounded subset, and reads a separate Git-created fixture. System Git is the only test oracle: it is never part of the shipped tool.

**Only a scaffold exists.** The repository holds the product plan, the technical specification, the backlog and an empty Rust crate: module stubs, a `gitcompat` entry point that answers `--help` and `--version`, and one smoke test each. No product behaviour is implemented; every other input exits 1 with "not implemented yet: see #11". Do not describe anything as working until a CI run shows it.

## Sources of truth

| Question | Where |
|---|---|
| What the product must do, scope, success, preflight | [docs/PRD.md](docs/PRD.md) |
| How it must behave: encodings, locking, limits, fixtures, expected IDs | [docs/TECHNICAL_SPEC.md](docs/TECHNICAL_SPEC.md) |
| Which module owns what (proposed, not implemented) | [docs/architecture.md](docs/architecture.md) |
| What to build next, in which order | Epic issue #1 and its child issues on GitHub |
| Agent roles and when each runs | "Agents and ticket work" below; provenance in [.claude/agency-agents/README.md](.claude/agency-agents/README.md) |
| Issue triage loop (manual, report-only) | [LOOP.md](LOOP.md) |
| How a ticket goes from the queue to a ready pull request, who may do what | "Delivery workflow" below |
| UI and copy rules | [docs/ui-quality.md](docs/ui-quality.md) |
| Decisions the owner made, with dates | [docs/owner-decisions.md](docs/owner-decisions.md) |

`docs/PRD.md` repeats the "Technical design" and "Deterministic fixture" text of `docs/TECHNICAL_SPEC.md`. If they differ, do not pick one silently: say so in the pull request and fix both in the same change. Where a ticket body and the specification differ, the specification wins and the ticket gets corrected.

## Stack and constraints

- Rust, standard library first. Minimal pinned crates, planned `sha1` and `flate2`, used only behind narrow SHA-1 and zlib interfaces. Licences and resolved sources are verified, `Cargo.lock` is committed, and the build works with `--locked --offline` from a vendored bundle or approved cargo cache.
- Offline: the build and every workflow run with the network blocked. No telemetry, no downloads at run time.
- SHA-1 object format and the files ref backend only. Regular files and directories, portable UTF-8 names, modes `100644`, `100755` and `40000`.
- Deferred, with a typed diagnostic and no change to the fixture: smart HTTP, remote credentials, linked and sparse worktrees, submodules, symlinks, SHA-256, reftable, rename detection, text-level conflict resolution, delta packs, packed refs, full CLI flag parity.
- Mutations and exports happen only inside tool-owned disposable fixture roots. Never write into a repository the tool did not create.
- Every read has an explicit limit (object, payload, path, graph, pack). Output is deterministic and carries a schema version.
- No licence has been selected for the project. Do not add one, and do not copy code from other projects: the plan requires an original implementation that keeps any upstream notice it adapts.
- No statement about speed until a measured report exists (PRD, ticket E44-6).

## Setup and commands

There is no Rust toolchain on the owner's PC and none may be installed for this work. Rust checks run in CI only. Git 2.47 is installed locally and can run the oracle commands.

| Purpose | Command | Status |
|---|---|---|
| Build | `cargo build --locked --offline` | real (scaffold); CI-only |
| Test | `cargo test --locked --offline` | real (scaffold smoke tests); CI-only |
| Format check | `cargo fmt --check` | real; CI-only |
| Lint | `cargo clippy --all-targets --locked --offline -- -D warnings` | real; CI-only |
| Run the tool | `cargo run --locked --offline -- <command>` (binary `gitcompat`; commands: `preflight`, `init`, `snapshot`, `inspect`, then `stage`, `status`, `ref`, `merge`, `export`, `pack-inspect`; spelling is the plan's default and is frozen by ticket #2, so follow the README once it exists) | `--help` and `--version` are real (scaffold); any other input exits 1 with "not implemented yet: see #11"; the commands are planned: #2 to #6 |
| Repeatable demo | the command sequence in the README | planned: introduced by ticket #2, extended by #7 |
| Oracle fixture | `git init --object-format=sha1 --ref-format=files --initial-branch main` | specified in docs/TECHNICAL_SPEC.md; used by tests from ticket #2 |
| Oracle checks | `git cat-file`, `git ls-tree -rz`, `git rev-parse`, `git status --porcelain=v1 -z`, `git fsck --strict`, `git verify-pack` | specified in docs/TECHNICAL_SPEC.md; used by tests from ticket #2 |

`.github/workflows/ci.yml` (jobs `test` and `rust`, Linux and Windows) runs the four cargo steps unconditionally; the guard is gone. The scaffold crate has no dependencies and `rust-toolchain.toml` pins `1.100.0` (stable of 2026-11-12, from the official Rust release notes), which CI installs from that file; ticket #2 re-checks the pin when it starts and ticket #10 adds `sha1` and `flate2` (vendored). The scaffold was never compiled locally, so the first CI run is its first proof.

Without a local toolchain, in every pull request:

- say plainly that the Rust checks were not run locally;
- leave the four Rust checks to the CI run of the draft pull request and mark it ready only when that run is green;
- never write "tests pass" for a command you did not see pass.

## Architecture boundaries

The proposed module map is [docs/architecture.md](docs/architecture.md). The rules it rests on, from the specification:

- The SHA-1 and zlib codecs sit behind narrow interfaces; no other code imports a third-party crate.
- Typed object storage is separate from commit parsing and tree traversal. Blobs are byte-exact: no filters, no line-ending changes, no hooks.
- Verify the header, size, stream boundaries and recomputed ID on every object read. Bound compressed and decoded sizes. Compare decoded bytes, never compressed streams.
- Tree order is Git's order (directories compare as if their names end in a slash). Reject unsupported modes, duplicates and malformed lengths.
- Publish completed objects before moving a ref. A ref moves through an exclusive lock, an expected-old check under the lock, a flushed write and a same-directory rename. A held lock or a stale expected value leaves the ref intact.
- Index v2: sorted stage-0 entries, padding, trailing checksum; reject unsupported stages, versions and mandatory extensions; replace atomically under a lock.
- Merge: bounded walk, one unambiguous base, path-level comparison. A conflict is reported as stable paths and writes nothing and moves nothing.
- Pack reader: read-only, pack v2 plus index v2, undeltified objects only; delta entries are rejected explicitly.
- Fixture paths: relative only; reject absolute and parent paths, symlinks and reparse points, reserved `.git` entries, duplicates and case or normalization collisions, before anything is written.
- The report is schema-versioned JSON through a bounded encoder with stable error codes; diagnostics go to a separate stream.
- The oracle runs Git with isolated config, templates, hooks, filters, identity and paging, in explicit scratch directories.

Changing any of these needs the owner's decision first; see the human gates in [LOOP.md](LOOP.md).

## Git workflow

- The default branch is `main`. The owner sets branch protection; the rules here apply on top of it.
- One branch and one pull request per issue. Branch from the tip of `origin/main`; name it after the issue, for example `issue-2-walking-skeleton`.
- Keep commits small, imperative, in English. Reference the issue in the pull request body; write `Closes #N` only when every acceptance criterion is met.
- Open pull requests as drafts. Only the owner merges. Do not push to `main`, force-push a shared branch, publish a crate (set `publish = false`), deploy or tag a release.
- Do not edit the owner's working checkout.

## Evidence before calling a ticket done

Put this in the pull request as pasted output or a link, not as a summary. The order is fixed: local checks, then Code Reviewer and Reality Checker on the full diff and the recorded evidence, then the draft pull request, then CI, then ready only when CI is green.

- Rust checks (`cargo fmt --check`, `cargo clippy ...`, `cargo build --locked --offline`, `cargo test --locked --offline`) cannot run on this PC. State that, and show that the CI job runs them; the CI result is checked afterwards and is not a gate input.
- Any check that can run locally (for example an oracle command against a fixture) with its exact output.
- For each acceptance criterion: the test name or fixture that proves it, and the oracle comparison where the ticket names one.
- Network-blocked run where the ticket requires it, and what was not verified and why.
- The review gate and the acceptance gate run on every ticket, whatever its size. The session or role that wrote a change does not count as its reviewer. Read-only roles cannot run commands: give them the diff, the acceptance criteria and the recorded output.
- List the roles that ran and their verdicts in the pull request description under "Agent roles run".
- CI green on the current diff (linked run), both platforms. It comes after the draft pull request exists; the driver checks it before `gh pr ready` and links the run in a comment on the issue. Reality Checker is not asked to confirm it.

## Agents and ticket work

The project roles are Claude Code subagents in `.claude/agents/`. Their source, licence and local changes are in [.claude/agency-agents/README.md](.claude/agency-agents/README.md). Each role file starts with a section "In this repository", written for this project; it and this file override the generic upstream text that follows it. Where a role section and this file differ, this file wins.

When asked to implement, continue or finish a GitHub issue, follow [.claude/skills/implement-ticket/SKILL.md](.claude/skills/implement-ticket/SKILL.md) (`/implement-ticket <issue number>` starts it by hand). Delegate to the roles without waiting to be asked; each description ends with a "Use proactively ..." sentence.

| When | Role | Access | Scope here |
|---|---|---|---|
| Before coding | Software Architect | read-only | a ticket that adds or changes a module boundary, storage interface, dependency or the CLI surface; checks the approach against the spec and [docs/architecture.md](docs/architecture.md) |
| While coding | Rust Refactoring Specialist | can edit | `src/` and `tests/` |
| While coding | Developer Tooling Engineer | can edit | CLI handling and report, `Cargo.toml`, `Cargo.lock`, vendor bundle, toolchain pin, CI cargo steps, demo |
| After coding | Application Security Engineer | read-only | parsers for objects, trees, commits, index and packs; limits; path validation; dependencies |
| Every ticket: review gate | Code Reviewer | read-only | the full diff, before the pull request is opened |
| Every ticket: acceptance gate | Reality Checker | read-only | each acceptance criterion against the evidence produced, after Code Reviewer |

- The review gate and the acceptance gate run on every ticket. The session that wrote a change does not count as its reviewer.
- Read-only roles cannot run commands: give them the diff, the acceptance criteria and the recorded output.
- List the roles that ran and their verdicts in the pull request description under "Agent roles run".

## Delivery workflow

How a ticket reaches the owner. Each piece is described in its own file; this part only links them.

- **Start:** the owner runs `/next-ticket` ([`.claude/skills/next-ticket/SKILL.md`](.claude/skills/next-ticket/SKILL.md)); it picks one leaf ticket (never a `tracking` parent or a ticket with open sub-issues; the last sub-issue's pull request also closes the parent), creates the branch and worktree, and runs the playbook [`.claude/skills/implement-ticket/SKILL.md`](.claude/skills/implement-ticket/SKILL.md). Nothing is scheduled.
- **Gate hook:** [`.claude/hooks/role_gate.py`](.claude/hooks/role_gate.py) blocks `gh pr create` and `gh pr ready` until Code Reviewer and Reality Checker ran after the last code edit. `ROLE_GATE=off` in the environment Claude Code is started from switches it off for that session; only the owner does that.
- **Permissions:** [`.claude/settings.json`](.claude/settings.json). Edits to CI, agent configuration and Loop files ask first, in the main checkout and in any ticket worktree (shell-made edits are not covered, so a ticket never changes agent configuration); force pushes, pushes to `main`, `gh pr merge`, releases and reads of `.env` are denied.
- **CI:** [`.github/workflows/ci.yml`](.github/workflows/ci.yml). Jobs `test` and `rust` must be green on the current diff before a pull request is marked ready. The agent does not poll: it ends its turn with "Draft pull request #N is waiting for CI" and reads `gh pr checks` once when woken. Reviewers read the diff and evidence from `.claude/state/review/`. An acceptance step that needs a person is listed under "Owner checks" in the pull request.
- **Pull request:** [`.github/pull_request_template.md`](.github/pull_request_template.md). Tickets use [`.github/ISSUE_TEMPLATE/implementation-ticket.md`](.github/ISSUE_TEMPLATE/implementation-ticket.md).
- **Labels:** `status:ready`, `status:blocked`, `status:in-progress`, `needs-decision`, `tracking`.
- **Changelog:** every ticket pull request adds one line under Unreleased in [CHANGELOG.md](CHANGELOG.md).
- **UI and copy:** [docs/ui-quality.md](docs/ui-quality.md); the `ui-ux-pro-max` and `unslop` skills are in `.claude/skills/`.
- **Decisions:** [docs/owner-decisions.md](docs/owner-decisions.md). Security reports: [SECURITY.md](SECURITY.md).
- **Merge and release:** only the owner merges and releases ([docs/RELEASING.md](docs/RELEASING.md)). An agent stops at a ready pull request.

## Loop

Issue triage runs by hand at L1, report-only: it rewrites `issue-triage-state.md` and appends to `loop-run-log.md`, and does nothing else. No scheduler exists. See [LOOP.md](LOOP.md).
