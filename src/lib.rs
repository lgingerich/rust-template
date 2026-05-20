//! A strict Rust project template.

/// Returns the crate name embedded by Cargo.
#[must_use]
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

#[cfg(test)]
mod tests {
    use super::crate_name;

    #[test]
    fn crate_name_is_available() {
        assert_eq!(crate_name(), "rust-template");
    }
}
