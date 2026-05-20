# Rust Template

A strict Rust project starter focused on safety, performance, and predictable tooling.

## Included

- Pinned stable Rust toolchain with `rustfmt` and `clippy`
- Strict Rust and Clippy lint configuration
- Release, profiling, and development Cargo profiles
- Stable `rustfmt.toml`
- Cargo backtraces for local development and tests
- GitHub Actions CI for format, lint, and test checks

## First Steps

1. Rename the package in `Cargo.toml`.
2. Update `authors`, `description`, `repository`, and `license`.
3. Replace `src/lib.rs` with your project code.
4. Run:

   ```bash
   cargo fmt
   cargo clippy --all-targets --all-features
   cargo test
   ```
