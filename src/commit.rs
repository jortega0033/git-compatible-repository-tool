//! `commit`: commit encode and parse; uses object, is not part of it.

/// One-line responsibility, copied from docs/architecture.md.
pub const RESPONSIBILITY: &str = "commit encode and parse; uses object.";

#[cfg(test)]
mod tests {
    #[test]
    fn module_links() {
        assert!(!super::RESPONSIBILITY.is_empty());
    }
}
