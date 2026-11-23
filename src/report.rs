//! `report`: schema-versioned JSON encoder, escaping, stable error codes, diagnostics stream.

/// One-line responsibility, copied from docs/architecture.md.
pub const RESPONSIBILITY: &str = "schema-versioned JSON encoder, stable error codes.";

#[cfg(test)]
mod tests {
    #[test]
    fn module_links() {
        assert!(!super::RESPONSIBILITY.is_empty());
    }
}
