# Git-compatible Repository Tool

Engineering plan for an original offline Rust CLI. The program builds an independent Git in six milestones ([#35](https://github.com/jortega0033/git-compatible-repository-tool/issues/35)). This document specifies milestone 1: a local compatibility subset in seven parent tickets and their sub-issues. The prototype, integration checks, and benchmarks are planned.

## Product requirements

A developer learning Git storage wants to create a small repository, see how files become commits, and verify the same history with installed Git. The developer starts by creating a disposable repository from synthetic inputs, inspects the commit and file contents with Git, then uses the tool to inspect a separate Git-created fixture.

**First working slice.** Create an empty SHA-1 fixture repository with a symbolic HEAD on main. Save one UTF-8 file as a blob, create the tree and first commit, and publish the main ref. Inspect the commit ID, tree, and file contents from the CLI. Verify the exact object IDs and reachable graph using independent Git commands. A clean offline checkout can reproduce the workflow.

**Epic workflow.** Save a nested snapshot, stage a subset of files, inspect staged and unstaged changes, and export a historical snapshot. Use two branches to reproduce a fast-forward, a clean two-parent merge of independent file changes, and a conflict reported without writing a commit or moving the ref. Inspect a separate fixture that Git has repacked within the declared pack subset.

**Scope of milestone 1.** Rust CLI, regular files and directories, portable UTF-8 names, SHA-1 object IDs, and the files ref backend. Support modes 100644, 100755, and 40000. Later milestones carry delta packs (M2), symlinks, rename detection and text-level conflict resolution (M3), smart HTTP (M4), and linked and sparse worktrees, submodules, SHA-256 and reftable (M5). Remote credentials and full CLI flag parity have no milestone yet. Restrict all mutations and exports to tool-owned disposable fixtures.

**Success.** Installed Git accepts the tool's supported objects, refs, index, and reachable history. The tool reads Git-created fixtures with identical file bytes and equivalent supported status and ref results. Each unsupported format produces a typed diagnostic and leaves the fixture unmodified.

## Technical design

### Offline runtime

Use Rust standard-library code for CLI, record parsing, paths, graph operations, and filesystem coordination. Place the SHA-1 and zlib codecs behind narrow interfaces. Before implementation verify and pin the selected minimal Rust dependencies, such as `sha1` and `flate2`, and their licenses and resolved sources. Stage a locked vendor bundle or an approved cargo cache, then verify `cargo build --locked --offline`. Report a tooling blocker if the target lacks those inputs. Use system Git only as the test oracle. The tool handles local object and ref operations itself.

Organize CLI commands around init, inspect, stage, status, named refs, limited merge, export, and pack inspect. Write a schema-versioned JSON report through a bounded encoder with an explicit string-escaping contract, sorted output where the input order carries no semantics, and stable error codes. Keep diagnostics in a separate stream. The CLI output is the inspectable interface for this epic.

### Object encodings

- Hash the ASCII type, a space, decimal payload byte length, a zero byte, and the exact payload. Store the zlib stream under `.git/objects/<2 hex digits>/<38 hex digits>`. Verify the header, size, stream boundaries, and recomputed ID on read. Bound both compressed and decompressed sizes. Compare decoded bytes rather than compressed streams. SHA-1 defines Git object identity in this format.
- Preserve blob bytes exactly including LF and binary content. The first epic applies no clean filters, line-ending transformations, or content hooks.
- A tree entry stores mode text, a space, basename, a zero byte, and 20 raw bytes of object ID. Use Git's ordering, treating directories as if their names end in a slash for comparison. Reject unsupported modes, duplicates, malformed lengths, and types inconsistent with the entry.
- A commit contains the tree line, zero or more ordered parent lines, author and committer lines, a blank line, and message bytes. Pin all identity and date fields for exact fixture IDs. Keep the typed object storage separate from commit parsing and tree traversal. Limit reads to unsigned commits within the declared subset.
- Publish completed immutable objects before moving a ref. Check an already-present object for byte equality and a matching ID. An interruption before ref publication leaves the old ref authoritative. [Git object storage](https://git-scm.com/book/en/v2/Git-Internals-Git-Objects) · [Git repository layout](https://git-scm.com/docs/gitrepository-layout)

### Index, refs, and merges

Read and write bounded SHA-1 index v2 files. Handle the header, sorted stage-0 entries, v2 padding, and trailing checksum. Populate relevant stat fields and derive changed-content checks from bounded rehashes of fixture files. Reject unsupported stages, index versions, and mandatory extensions. Validate extension lengths before skipping supported optional data. Update the index using a local lock and atomic replacement. Compare normalized status records with `git status --porcelain=v1 -z`. [Git index format](https://git-scm.com/docs/gitformat-index) · [Git porcelain status](https://git-scm.com/docs/git-status)

Use symbolic HEAD and loose `refs/heads/**` in the writable subset. Validate names and prefix collisions. To move a single ref, create its lock exclusively, check the expected prior value under lock, write and flush the replacement, and rename within the same directory. Lock contention and a stale expected value leave the ref intact. Publish only after every reachable object has been written. Test process interruption separately from disk or power-loss durability. Use a typed diagnostic for packed refs. [Git init options](https://git-scm.com/docs/git-init) · [Git ref updates](https://git-scm.com/docs/git-update-ref) · [Git ref-name rules](https://git-scm.com/docs/git-check-ref-format)

Compute ancestry with a bounded graph walk. Handle fast-forwards and a simple three-way merge with one unambiguous base. Compare complete path entries against the base. Combine unchanged and independent changes. Report add/add, delete/modify, file/directory, and simultaneous distinct edits as stable conflict paths. A clean merge writes the tree and ordered two-parent commit before moving the ref with the expected-old guard. Materialize snapshots only into fresh output directories. Compare file bytes and parent order with Git, with exact commit hashes reserved for fixtures that pin identity, timestamp, and message.

### Pack reading and filesystem rules

Support read-only access to bounded pack v2 and index v2 pairs with undeltified objects. Validate the index fanout, sorted IDs, offsets, cross-file checksums, and per-entry CRCs. Validate decoded lengths and recompute object IDs. Reject delta entries and unsupported storage explicitly. Generate the independent fixture with Git delta selection disabled and verify the fixture's entry types before running the planned test. Keep pack writing for later work. [Git pack and index formats](https://git-scm.com/docs/gitformat-pack)

Allocate one exclusive disposable fixture root per test run. Resolve relative paths within that root and reject absolute or parent paths, symlinks and reparse points, reserved .git entries, duplicate paths, and ambiguous case or normalization collisions. Validate every materialization path before writing into a fresh subdirectory. Use explicit scratch working directories and isolated Git config, templates, hooks, filters, identity, and paging settings for the oracle. The fixture root is tool-owned and exclusively controlled.

## Deterministic fixture and verification

Git hash-object and an independent SHA-1 calculation agreed on these values on the planning host. The future Rust tool must reproduce them.

| Object | Fixture | Payload bytes | Expected SHA-1 |
|---|---|---:|---|
| Blob | ASCII hello followed by one LF | 6 | `ce013625030ba8dba906f756967f9e9ca394464a` |
| Tree | One `100644 hello.txt` entry, NUL and 20 raw blob-ID bytes | 37 | `aaa96ced2d9a1c8e72c56b253a0e2fe78393feb7` |
| Commit | The tree and fixed identity below, with no parents | 199 | `115b3c5eb93781da313bc1d1ba391a20c20a029b` |

Use LF line endings and terminate the message with one LF.

~~~
tree aaa96ced2d9a1c8e72c56b253a0e2fe78393feb7
author Fixture Person <fixture@example.test> 1704067200 +0000
committer Fixture Person <fixture@example.test> 1704067200 +0000

First synthetic snapshot
~~~

Initialize native fixtures using `git init --object-format=sha1 --ref-format=files --initial-branch main` where the installed Git version supports all options. Verify the resulting formats. Compare both directions using `git cat-file`, `git ls-tree -rz`, `git rev-parse`, `git status --porcelain=v1 -z`, and `git fsck --strict` in tool-owned fixture directories. Preserve path bytes and parent order when normalizing results. [Git cat-file](https://git-scm.com/docs/git-cat-file) · [Git fsck](https://git-scm.com/docs/git-fsck)

For merge checks, create a shared base over left.txt and right.txt. Change left.txt on branch A and right.txt on branch B. A successful merge retains both changes and both ordered parents. Change left.txt differently on both branches in a separate fixture and expect a path-specific conflict with no ref movement. Add separate fast-forward and file/directory collision cases. [Git merge bases](https://git-scm.com/docs/git-merge-base)

## Epic E44 · Round-trip local snapshots across Git and the Rust tool

Deliver the documented compatibility subset, an accessible CLI demo, and an independent test harness. The epic passes when Git recognizes the tool's objects, refs, index, and clean merge histories, and when the tool reads the declared native Git fixtures with matching contents and status. Get repeatable diagnostics for unsupported formats, conflicts, and invalid paths without changes to the source fixture.

| Ticket | Depends on | Estimate |
|---|---|---:|
| E44-1 Loose objects and first snapshot | — | 3–5 h |
| E44-2 Nested snapshots, refs, and export | E44-1 | 3–5 h |
| E44-3 Index v2, staging, and status | E44-1 | 4–6 h |
| E44-4 Ancestry, fast-forward, and bounded merge | E44-2 | 3–5 h |
| E44-5 Bounded Git pack reader | E44-1 | 4–6 h |
| E44-6 Offline integration and compatibility report | E44-2, E44-3, E44-4, E44-5 | 3–5 h |

Freeze the object-store interface and fixture-path rules in E44-1. Then E44-2, E44-3, and E44-5 can run in parallel on distinct modules and fixture roots. E44-4 follows E44-2. Keep storage dispatch integration with the pack lane and shared CLI wiring with the lead. E44-6 integrates all lanes.

### E44-1 · Loose objects and first snapshot

Implement offline preflight, fixture-only init, blob/tree/commit encoding, bounded zlib storage, and one-file inspection. **Acceptance and verification.** Match all three pinned IDs. Git reads the file and accepts the object graph, and the tool reads a Git-created copy with the same bytes. A truncated object and one exceeding the decoded size cap fail with distinct errors. Run the workflow with network access blocked.

### E44-2 · Nested snapshots, refs, and export

Add recursive tree enumeration, fixture branch creation and expected-old ref updates, and read-only snapshot export. **Acceptance and verification.** Compare a nested binary file, an executable fixture on a mode-aware host, and a directory-prefix ordering fixture with Git. Interrupt a ref update before replacement, reopen, and verify the old ref and its reachable objects. A stale writer or a held lock leaves the ref untouched. A symlink or file/directory collision fails before writing the export.

### E44-3 · Index v2, staging, and status

Implement index checksum verification, bounded entry reading and writing, and stage-0 replacement under a lock. **Acceptance and verification.** In a three-file fixture, stage a change in one tracked file while leaving another modified and a third untracked. Compare normalized status with Git in both directions. Reopen after a successful stage. Reject unsupported versions or a bad checksum, and preserve an existing valid index if a proposed replacement fails validation.

### E44-4 · Ancestry, fast-forward, and bounded merge

Implement ancestry search, one-base file-level comparison, fast-forward ref movement, and a two-parent commit for clean independent edits. **Acceptance and verification.** Git recognizes the resulting ancestry, preserves both file changes, and reports the expected parent order. Refuse ambiguous merge bases. Verify typed add/add, file/directory, delete/modify, and distinct same-path change conflicts. A conflicted preview leaves both ref and source fixture intact.

### E44-5 · Bounded Git pack reader

Read small non-delta pack v2 and index v2 pairs into the same object reader interface. **Acceptance and verification.** Use Git-generated fixtures whose entry types are confirmed by `git verify-pack`. Compare decoded bodies with `git cat-file` before and after the fixture switches from loose to pack-only access. Reject an out-of-bounds offset, a checksum mismatch, and a delta entry before reporting successful inspection. [Git verify-pack](https://git-scm.com/docs/git-verify-pack)

### E44-6 · Offline integration and compatibility report

Integrate both directions of snapshot, staging, status, merge, conflict, and pack inspection checks. **Acceptance and verification.** Run the documented workflow twice offline from a clean checkout with staged build inputs. Record versions, fixture sizes, pass and error outcomes, and measured runtime. Deliberately change one expected result and ensure the harness identifies the operation and object or path that diverged. Report unsupported formats and avoid unmeasured throughput claims.

## Implementation preflight

Confirm the target Rust and Git versions, approved codec dependencies, and writable native scratch directory. Explicitly set the supported format and object, payload, path, graph, and pack limits. Verify same-directory rename behavior on the target. The first compatibility report records the exact supported versions, fixture format, and observed checks. [Official project inspiration](https://docs.google.com/document/d/1R5MyfVvn033KoXZXgXP65qRAXnhcjtVvO_VF7IxUZaA/edit?tab=t.ef9a30wzb1z5)
