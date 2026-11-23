//! `limits`: the explicit object, payload, path, graph, pack and index limits.

/// One-line responsibility, copied from docs/architecture.md.
pub const RESPONSIBILITY: &str = "the explicit object, payload, path, graph and pack limits.";

#[cfg(test)]
mod tests {
    #[test]
    fn module_links() {
        assert!(!super::RESPONSIBILITY.is_empty());
    }
}
