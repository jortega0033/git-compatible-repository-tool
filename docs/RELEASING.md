# Releasing

Only the owner releases. Agents never tag, publish or create a GitHub release.

## What a release is here

Nothing is implemented yet and no licence is selected, so there is no release. Once the crate exists (ticket #2) a release is a Git tag `vMAJOR.MINOR.PATCH` on a commit of `main`, matching `version` in `Cargo.toml`. The crate is not published (`publish = false`), nothing is deployed and there is no packaged artifact beyond what a clone builds with `cargo build --locked --offline`. Do not tag before epic #1 is complete and a licence decision exists.

## Checks before tagging

1. CI is green on the tip of `main` for both jobs (`test` and `rust`) on Linux and Windows.
2. `cargo fmt --check`, `cargo clippy --all-targets --locked --offline -- -D warnings`, `cargo build --locked --offline` and `cargo test --locked --offline` pass, from that CI run.
3. The README demo reproduces the three pinned IDs of `docs/TECHNICAL_SPEC.md` twice from a clean checkout, and `docs/compatibility-report.md` (ticket #7) is current.
4. `Cargo.lock` is committed and `docs/dependencies.md` lists every vendored crate with its licence.
5. `CHANGELOG.md`: move the `Unreleased` entries under a new version heading with the date.

## Tagging

1. In a pull request, set `version` in `Cargo.toml` (and `Cargo.lock`) and update `CHANGELOG.md`; the owner merges it.
2. On the merge commit: `git tag -a vX.Y.Z -m "vX.Y.Z"` and `git push origin vX.Y.Z`.
3. Optional: create a GitHub release from the tag with the changelog section as its text.
