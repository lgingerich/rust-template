# Rust Template

A strict Rust project starter focused on safety, performance, and predictable tooling.

## Included

- Pinned stable Rust toolchain with `rustfmt` and `clippy`
- Strict Rust and Clippy lint configuration
- Release, profiling, and development Cargo profiles
- Stable `rustfmt.toml`
- Cargo backtraces for local development and tests
- GitHub Actions CI for format, lint, and test checks
- Build caching via [`kache`](https://github.com/kunobi-ninja/kache), set as the `rustc-wrapper` in `.cargo/config.toml`

## First Steps

1. Install the build cache wrapper:

   ```bash
   cargo install kache --locked
   ```

   `kache` is set as `rustc-wrapper` in `.cargo/config.toml`, so it must be on
   `PATH` to build. To build without it, run `RUSTC_WRAPPER= cargo build`.

2. Rename the package in `Cargo.toml`.
3. Update `description` and `repository` in `Cargo.toml`. Keep the MIT license or
   update both `license` and `LICENSE` to match your project's terms. Preserve
   existing copyright notices on retained template material and add your own
   for your contributions.
4. Replace `src/lib.rs` with your project code.
5. Run:

   ```bash
   cargo fmt
   cargo clippy --all-targets --all-features
   cargo test
   ```

## License

MIT. See [LICENSE](./LICENSE) for the full text.
