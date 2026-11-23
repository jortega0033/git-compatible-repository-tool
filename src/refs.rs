//! `refs`: symbolic HEAD, loose refs/heads/**, lock and expected-old update.

/// One-line responsibility, copied from docs/architecture.md.
pub const RESPONSIBILITY: &str = "symbolic HEAD, loose branch refs, lock, expected-old update.";

#[cfg(test)]
mod tests {
    #[test]
    fn module_links() {
        assert!(!super::RESPONSIBILITY.is_empty());
    }
}
