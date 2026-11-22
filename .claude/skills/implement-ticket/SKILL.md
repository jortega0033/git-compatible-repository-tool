---
name: implement-ticket
description: Playbook for implementing a GitHub issue in this repository from start to pull request, with the project subagents run at fixed steps. Use whenever asked to implement, start, continue or finish a ticket, an issue or an epic child, for example "implement #12" or "work on the next ticket".
argument-hint: "[issue number]"
---

# Implement a ticket

Use this playbook for every implementation ticket in this repository. `CLAUDE.md` holds the commands, the architecture boundaries, the Git workflow and the evidence rules; where this playbook and `CLAUDE.md` differ, `CLAUDE.md` wins.

The project subagents in `.claude/agents/` are part of the work. Delegate to them at the steps below without waiting to be asked. If a step names a role that is not installed, do that step yourself and say so in the pull request.

## Steps

1. **Read the ticket.** `gh issue view <number>` and the epic it belongs to. If it is a `tracking` ticket or has open sub-issues, refuse it, list its open sub-issues and stop. Also stop if the ticket is closed, `status:blocked`, `status:in-progress` or `needs-decision`. Check that every prerequisite issue is closed and that no owner decision named in the ticket is still open. If the ticket is blocked, stop and say what blocks it.
2. **Read the rules.** `CLAUDE.md`, then the architecture notes it links to.
3. **Orient and check the approach.** Delegate as the table "Before coding" says. Settle open design questions before writing code. If the ticket adds a screen or page (none is planned), start with the `ui-ux-pro-max` skill (`python .claude/skills/ui-ux-pro-max/scripts/search.py`) and read [`docs/ui-quality.md`](../../../docs/ui-quality.md).
4. **Branch** as the Git workflow in `CLAUDE.md` says. Never work on the default branch.
5. **Implement** inside the ticket's stated scope. Delegate the work to the implementing role that matches the files being changed (table "While coding"). Do not widen scope; note anything else you find as a follow-up. Do not change agent configuration (`.claude/`) in a ticket.
6. **Test.** Add or change tests for every acceptance criterion and material edge case. The Rust checks in `CLAUDE.md` cannot run on this PC (no toolchain): run any check that can run locally, such as an oracle command against a fixture, and record each command and its result; say plainly that the cargo checks were not run locally and that job `rust` of `.github/workflows/ci.yml` runs them.
7. **UI and writing check.** Check any changed command output, message or screen against `docs/ui-quality.md`. Run the `unslop` skill over user-facing text, documentation and the pull request text before the gates, so they judge the final diff.
8. **Changelog.** Add one line to `CHANGELOG.md` under Unreleased, before the gates, so they judge the final diff.
9. **Specialist review.** Run every role in the table "After coding" whose trigger matches the change.
10. **Review gate, always.** Write the diff and the local evidence to `.claude/state/review/` (`git diff --stat origin/main...HEAD`, `git diff origin/main...HEAD`, the check outputs) and name those files in the request. Run Code Reviewer on the full diff. Fix every material finding, re-run the checks, and have the reviewer look again until no material finding is left.
11. **Acceptance gate, always.** Run Reality Checker with the ticket's acceptance criteria and the local evidence you produced. It checks that the documented checks ran locally where possible and that CI is configured to run them; do not ask it to confirm a CI result, which does not exist yet. Do not open the pull request unless it passes.
12. **Hand off.** Capture the evidence `CLAUDE.md` asks for. Then open the pull request as a draft from `.github/pull_request_template.md`, naming the issue, only after both gates passed on the current diff (the gate hook enforces this). A step that needs a person (marked manual in the ticket) is done as far as an agent can and the rest is listed under "Owner checks". Wait for CI on the draft without polling: end the turn with "Draft pull request #N is waiting for CI" and read `gh pr checks` once when woken. If it fails, fix, re-run the local checks and both gates, and push. Mark the pull request ready (the hook checks the gates again) only when CI is green on the current diff, and link the run in a comment on the issue. The last open sub-issue of a parent also closes the parent (`Closes #<parent>`). The owner merges; never merge it yourself.

## Roles

### Before coding

| Role | Access | Delegate to it |
|---|---|---|
| Software Architect | read-only reviewer | Before implementing a ticket that adds or changes a module boundary, data model, storage schema, public API or dependency, to check the approach against the architecture notes |

### While coding

| Role | Access | Delegate to it |
|---|---|---|
| Developer Tooling Engineer | can edit | When a ticket changes build scripts, CI, toolchain pins or developer commands |
| Rust Refactoring Specialist | can edit | When a ticket changes Rust source or Rust tests |

### After coding

| Role | Access | Delegate to it |
|---|---|---|
| Application Security Engineer | read-only reviewer | After a ticket touches input parsing, file upload or import, sanitization, origin and host checks, or any handling of untrusted data |

### Gates that run on every ticket

| Role | Access | Delegate to it |
|---|---|---|
| Code Reviewer | read-only reviewer | After implementing or changing code for a ticket, before the work is called done or a pull request is opened |
| Reality Checker | read-only reviewer | As the acceptance gate of every ticket, after Code Reviewer and before the draft pull request is opened, to check each acceptance criterion against evidence that was actually produced |

## Rules

- The session or role that wrote a change does not count as its reviewer.
- Read-only reviewers cannot run commands. Give them the diff, the acceptance criteria and the recorded command output.
- Where a role's general advice (frameworks, dependencies, cloud services) conflicts with `CLAUDE.md`, `CLAUDE.md` wins.
- In the pull request description, add a section "Agent roles run" listing each role that ran, what it was asked and its verdict, and any step you did yourself because a role was missing.
