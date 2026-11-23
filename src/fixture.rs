//! `fixture`: fixture root, tool-owned marker and relative-path validation.

/// One-line responsibility, copied from docs/architecture.md.
pub const RESPONSIBILITY: &str = "fixture root, ownership marker, relative-path validation.";

#[cfg(test)]
mod tests {
    #[test]
    fn module_links() {
        assert!(!super::RESPONSIBILITY.is_empty());
    }
}
