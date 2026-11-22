## Ticket

Closes #

## What changed

## Evidence

Each command and its observed result (pasted output or a CI run link). The Rust checks are not run on the owner's PC: link the CI run (rustc and git versions are printed in its log) and say they were not run locally.

| Command | Result |
|---|---|
| | |

## Owner checks

Steps the ticket marks as manual: what the agent did and recorded, and what remains for the owner. Write 'None' if there are none.

## Agent roles run

| Role | What it was asked | Verdict |
|---|---|---|
| Code Reviewer | | |
| Reality Checker | | |

## Risks and rollback

## Checklist

- [ ] CI is green on the current diff
- [ ] Both gate roles ran after the last code edit
- [ ] Docs, fixtures and README changed in the same pull request where behaviour changed
- [ ] `docs/ui-quality.md` checked for any changed command output or message text
- [ ] Copy and pull request text went through the `unslop` skill
- [ ] `CHANGELOG.md` has a line under Unreleased
- [ ] No dependency was added beyond `sha1` and `flate2`
