# Agency Agents roles used in this repository

The Markdown files in `.claude/agents/` whose names start with `engineering-`, `security-` or `testing-` are copies of role definitions from Agency Agents. They are project-local Claude Code subagents. The file `loop-verifier.md` in the same folder is not from Agency Agents; it belongs to the Loop Engineering setup described in [`LOOP.md`](../../LOOP.md).

## Source and licence

- Upstream: https://github.com/msitarzewski/agency-agents
- Pinned revision: `8ae6164602c95ca78828b148b9052460ee426885` (upstream `main`, committed 2026-11-09)
- Licence: MIT, "Copyright (c) 2025 AgentLand Contributors". The upstream licence text at the pinned revision is kept unchanged in [`LICENSE`](LICENSE) next to this file (SHA-256 `9a45258434d5cedf0af73c9ad4771373701225038d246c49219026c33677f66f`). It covers only the copied role files, not the rest of this repository.
- Fetched with `gh api -H "Accept: application/vnd.github.raw" "repos/msitarzewski/agency-agents/contents/<path>?ref=<revision>"`.

## Installed roles

"Upstream SHA-256" is the hash of the file as fetched at the pinned revision. Reviewer roles have one added frontmatter line, `tools: Read, Grep, Glob`, so they cannot edit files or run commands. Nothing else in any file was changed at install time.

| File in `.claude/agents/` | `name:` | Upstream path | Upstream SHA-256 | Local change |
|---|---|---|---|---|
| `engineering-software-architect.md` | Software Architect | `engineering/engineering-software-architect.md` | `b85121691776147bacde26673e68fbbacd6a7f0816d1ca8e203c264d25ce1d30` | added `tools: Read, Grep, Glob` |
| `engineering-code-reviewer.md` | Code Reviewer | `engineering/engineering-code-reviewer.md` | `7509fcc3ea1dda46511b2801996305bf3d4a125576f9655ff0b63df25602f446` | added `tools: Read, Grep, Glob` |
| `testing-reality-checker.md` | Reality Checker | `testing/testing-reality-checker.md` | `6d32fcdb114233e13902ec6372d50293b120e85d490b5e81d372c29808f988a1` | added `tools: Read, Grep, Glob` |
| `security-appsec-engineer.md` | Application Security Engineer | `security/security-appsec-engineer.md` | `a19f0bda46f4adca5737365dbc3c7ed93e9d93b71349536d92ff055dcf4e3b83` | added `tools: Read, Grep, Glob` |
| `engineering-rust-refactoring-specialist.md` | Rust Refactoring Specialist | `engineering/engineering-rust-refactoring-specialist.md` | `06ca53dbd12694560bf01feb94cbbd8c428e4abe6ac92c8a72eeb277ce179a78` | none |
| `engineering-developer-tooling-engineer.md` | Developer Tooling Engineer | `engineering/engineering-developer-tooling-engineer.md` | `948d44fddfa47e2363236de150f9dc948edd79599609a92af8bf6b72b5b9b609` | none |

## Why these roles

This repository is planning-only: there is no code yet. The roles are chosen for the planned Rust command-line tool described in [`docs/TECHNICAL_SPEC.md`](../../docs/TECHNICAL_SPEC.md).

| Role | Use in this repository |
|---|---|
| Software Architect (read-only) | Check a ticket's design against the planned module boundaries in [`docs/architecture.md`](../../docs/architecture.md): the narrow SHA-1 and zlib interfaces, typed object storage separate from commit parsing and tree traversal, the ref and index lock protocols. |
| Code Reviewer (read-only) | Review a diff for correctness, missed edge cases and broken rules (bounded reads, publish-objects-before-refs, fixture-only mutation, byte-exact blobs). |
| Reality Checker (read-only) | Compare claimed evidence with what was actually run, including the oracle comparisons against system Git, before a ticket is called done. |
| Application Security Engineer (read-only) | Review the places where untrusted bytes or paths meet the tool: object, tree, commit, index and pack parsers, size and graph limits, and path validation for export and fixture roots. |
| Rust Refactoring Specialist | Implement and restructure the Rust source and tests. |
| Developer Tooling Engineer | Implement the command-line surface (arguments, JSON report, exit codes, diagnostics stream), the pinned toolchain, the offline vendor bundle and the build and CI commands. |

Most of these roles are written for larger stacks (frameworks, cloud services, scanners, pipelines). In this repository the rules in [`CLAUDE.md`](../../CLAUDE.md) win: Rust, standard library first, minimal pinned crates, offline, system Git only as the test oracle.

Roles that were considered and left out: Test Automation Engineer (written for Playwright and Cypress; this is a command-line tool), Performance Benchmarker (the plan forbids unmeasured throughput claims and sets no budgets), Technical Writer, Codebase Onboarding Engineer (there is no code to trace yet; add it when code exists).

## Name collision with user-level agents

A developer who has installed Agency Agents in `~/.claude/agents/` already has agents with the same `name:` values. Claude Code gives the project-level definition precedence, so inside this repository the copies here are used, including the read-only tool limits. The upstream names were kept on purpose so that the roles stay recognisable and can be compared with upstream. A Claude Code session that was already running when these files were added keeps the definitions it loaded at start: start a new session to pick up the project copies.

## Check that the roles load

From the repository root, in a new session:

```text
claude -p "List the project-level subagents defined in .claude/agents with the tools each one may use."
```

The four reviewer roles and `loop-verifier` must report exactly `Read, Grep, Glob`. The check was not run when the roles were installed (superseded: see the verification lines dated 2026-11-22 below).

## Updating

Fetch the same paths at a newer revision, re-apply the `tools:` line to the four reviewer roles, and update the revision and hashes in this file in the same commit.

## Local change: automatic delegation (2026-11-22)

One sentence beginning "Use proactively" was appended to the `description:` line of every Agency Agents role in `.claude/agents/` (not to `loop-verifier`). Claude Code uses that field to decide when to delegate, so the roles now run during ticket work without being named. Nothing else in the role files changed.

A section headed "In this repository" was then inserted directly after the frontmatter of every Agency Agents role (not into `loop-verifier`), before the upstream text. It is project-specific text written for this repository, not upstream text: the project and its planned stack, the files the role owns or reviews, its planned commands (or the output it must be given when it is read-only), a checklist of this project's rules with their source documents, the parts of the generic text that do not apply here, and what the role hands back. It ends with the sentence "This section and CLAUDE.md override the generic text below wherever they differ." The repository has no code yet, so every stack, module and command in these sections is marked as planned.

This supersedes any statement above that a role file is unmodified or that the `tools:` line is the only local change. To compare a role with the pinned upstream file, remove the "Use proactively" sentence (in a description that has no final full stop upstream, also the full stop that was added in front of it), the section "In this repository" together with the blank line after it, and the `tools:` line where one was added; the result then has the upstream SHA-256 listed above.

Verification: not run in this pass (superseded by the verification lines dated 2026-11-22 under "Local change: SDLC kit" and "Local change: loop-verifier" below).

The steps at which each role runs are in `.claude/skills/implement-ticket/SKILL.md`.

## Local change: SDLC kit (2026-11-22)

This pass added, without changing any role definition except the lines named below:

- The review gate hook `.claude/hooks/role_gate.py` (gate roles Code Reviewer and Reality Checker, both installed) and `.claude/settings.json`.
- The `next-ticket` driver, the `ui-ux-pro-max` and `unslop` skills, CI, pull request and issue templates, `docs/ui-quality.md` and `docs/owner-decisions.md`. See `CLAUDE.md` "Delivery workflow".
- Code Reviewer and Reality Checker: the caller writes the diff and evidence to `.claude/state/review/` and names the files. Code Reviewer: one checklist line for `docs/ui-quality.md`. Developer Tooling Engineer: one line saying the CI workflow exists. These lines sit inside the "In this repository" section and are removed with it when comparing a role with upstream.

Verification, 2026-11-22 (after the SDLC kit): one headless run from the worktree root with Claude Code 2.1.323, `claude -p "List every project subagent in .claude/agents with its name and tools, say whether each starts with a section titled In this repository, and list the project skills." --model haiku`. The answer listed seven agents: Code Reviewer, Reality Checker, Application Security Engineer and Software Architect with `Read, Grep, Glob`; Rust Refactoring Specialist, Developer Tooling Engineer and `loop-verifier` with no `tools` line. Six had an "In this repository" section; `loop-verifier` did not. It listed eight project skills: implement-ticket, next-ticket, issue-triage, loop-budget, loop-constraints, loop-intake, unslop, ui-ux-pro-max. The run also printed two warnings: the allow rules of `.claude/settings.json` were ignored because the workspace was not yet trusted (the worktree had never been opened interactively), and the `Write(**/.claude/...)` ask rules are not matched by file permission checks, since Claude Code applies only `Edit(...)` rules to file edits (`Edit` rules already cover the same paths). The `Write` entries come from the shared kit template and were left unchanged. The answer was not checked against a launched role.

## Local change: loop-verifier read-only tools (2026-11-22)

`loop-verifier.md` came from the Loop Engineering scaffold, not from Agency Agents. Its generated frontmatter had no `tools:` line, so it inherited every tool, which contradicted `LOOP.md` and the list above. The line `tools: Read, Grep, Glob` was added, and its instruction to run the tests itself was replaced by "answer `ESCALATE_HUMAN` if the verdict depends on a test run you cannot see recorded output for". Details are in `LOOP.md` "Verifier".

Verification, 2026-11-22: one headless run from the worktree root with Claude Code 2.1.323, `claude -p "List every project subagent in .claude/agents with its name and tools. Answer as a short list." --model haiku`. The answer listed seven agents. Code Reviewer, Reality Checker, Application Security Engineer, Software Architect and `loop-verifier` reported `Read, Grep, Glob`; Rust Refactoring Specialist and Developer Tooling Engineer reported no `tools:` line (they inherit all tools, as intended for the implementer roles). The run was not checked against a launched role.
