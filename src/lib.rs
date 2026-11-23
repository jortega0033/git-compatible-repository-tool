//! gitcompat: an offline, Git-compatible repository tool for a bounded subset of Git.
//!
//! Scaffold only: the modules below follow docs/architecture.md and hold no behaviour yet.
//! Modules `index`, `merge`, `pack` and `export` are added by tickets #4 to #6 and #3.

pub mod cli;
pub mod commit;
pub mod fixture;
pub mod hash;
pub mod limits;
pub mod object;
pub mod refs;
pub mod report;
pub mod tree;
pub mod zlib;
