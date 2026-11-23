//! `hash`: the only code that touches the sha1 crate, behind a narrow interface.

/// One-line responsibility, copied from docs/architecture.md.
pub const RESPONSIBILITY: &str = "the only code that touches the sha1 crate.";

#[cfg(test)]
mod tests {
    #[test]
    fn module_links() {
        assert!(!super::RESPONSIBILITY.is_empty());
    }
}
