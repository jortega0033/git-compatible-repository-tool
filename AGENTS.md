# AGENTS.md

The full repository instructions are in [CLAUDE.md](CLAUDE.md). This file repeats only what an agent needs before it reads that.

## Status

Planning only: there is no code, no `Cargo.toml` and no test yet. The commands in `CLAUDE.md` are planned and name the ticket that introduces each. There is no Rust toolchain on the owner's PC; Rust checks run in CI only.

## Planned test command

```
cargo test --locked --offline
```

Planned: introduced by ticket #2. Rust, standard library first, minimal pinned crates behind narrow interfaces, offline. Do not add dependencies beyond the ones the plan names without the owner's decision.

## Ticket work

To implement a GitHub issue, follow the steps in [.claude/skills/implement-ticket/SKILL.md](.claude/skills/implement-ticket/SKILL.md) and the section "Agents and ticket work" in [CLAUDE.md](CLAUDE.md). The role files in `.claude/agents/` each start with a section "In this repository" that holds the project rules for that role. The review gate and the acceptance gate apply to every ticket, and the agent that wrote the change cannot pass them. Only the owner merges.

## Loop conventions

- The loop runs at L1: report-only, started by hand. It proposes; it does not label, comment, close, edit source, push or merge.
- No scheduler and no loop workflow is installed. Do not add one without the owner's approval.
- See [LOOP.md](LOOP.md) for the manual command and the human gates.
