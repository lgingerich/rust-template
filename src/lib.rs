//! A strict Rust project template.

/// Returns the crate name embedded by Cargo.
#[must_use]
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
