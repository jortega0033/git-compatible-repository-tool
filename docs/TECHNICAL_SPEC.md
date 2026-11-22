# Git-compatible Repository Tool: technical specification

Planning document. Implementation and verification remain future work.

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
