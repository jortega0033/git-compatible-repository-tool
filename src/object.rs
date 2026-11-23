//! `object`: typed object storage (type, size, bytes, ID); loose backend, backend dispatch.

/// One-line responsibility, copied from docs/architecture.md.
pub const RESPONSIBILITY: &str = "typed object storage: type, size, bytes, ID; loose backend.";

#[cfg(test)]
mod tests {
    #[test]
    fn module_links() {
        assert!(!super::RESPONSIBILITY.is_empty());
    }
}
