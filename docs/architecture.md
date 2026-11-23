# Implementer map (scaffolded, behaviour not implemented)

The scaffold adds `src/lib.rs` (library `gitcompat`), a thin `src/main.rs` and a stub for `cli`, `report`, `limits`, `fixture`, `hash`, `zlib`, `object`, `tree`, `commit` and `refs`; each stub holds only its responsibility line and a link test. `index`, `merge`, `pack`, `export` and `tests/oracle` do not exist yet.

Nothing in this file exists in the repository yet. It maps the boundaries that [`TECHNICAL_SPEC.md`](TECHNICAL_SPEC.md) already requires onto proposed Rust modules, so that the tickets in epic #1 can work on separate files. The first ticket (#2) freezes the real layout and must update this file in the same pull request; where this file and the code disagree after that, the code wins and this file is fixed.

The specification stays the source of truth for behaviour. This file adds no rules.

## Proposed modules

| Module (proposed) | Owns | Spec section | First ticket |
|---|---|---|---|
| `cli` | argument parsing, command dispatch, exit codes | Offline runtime | #2 |
| `report` | schema-versioned JSON encoder, string escaping, stable error codes, separate diagnostics stream | Offline runtime | #2 |
| `limits` | the explicit object, payload, path, graph, pack and index limits | PRD "Implementation preflight" | #2 |
| `fixture` | fixture root, tool-owned marker, path validation (relative only, no `..`, no reserved `.git`, no symlink or reparse point, case and normalization collisions) | Pack reading and filesystem rules | #2, extended by #3 and the diagnostics ticket (E44-7, #8) |
| `hash`, `zlib` | the only code that touches the `sha1` and `flate2` crates; narrow interfaces | Offline runtime | #2 |
| `object` | typed object storage: type, size, bytes, ID; loose backend; dispatch to other backends | Object encodings | #2 |
| `tree`, `commit` | encode and parse; they use `object`, they are not part of it | Object encodings | #2, #3 |
| `refs` | symbolic HEAD, loose `refs/heads/**`, lock, expected-old update, packed-refs diagnostic | Index, refs, and merges | #2 (publish), #3 |
| `index` | index v2 read and write, checksum, lock and atomic replacement | Index, refs, and merges | #4 |
| `merge` | bounded ancestry walk, fast-forward, one-base three-way merge, conflict paths | Index, refs, and merges | #5 |
| `pack` | read-only pack v2 and index v2 reader behind the `object` interface | Pack reading and filesystem rules | #6 |
| `export` | materialize a snapshot into a fresh directory | Index, refs, and merges | #3 |
| `tests/oracle` (test support) | runs system Git with isolated config, templates, hooks, filters, identity and paging; normalizes its output | Deterministic fixture and verification | #2 |

## Boundaries that must hold

- Only `hash` and `zlib` import third-party crates. Everything else uses the standard library.
- `object` knows bytes, types and IDs. It does not parse commits or walk trees.
- A ref moves only after every object it reaches is written (`refs` is called last).
- Every read has a limit from `limits`; every mutation is inside a tool-owned fixture root.
- System Git appears only under `tests/`, never in the shipped tool.
