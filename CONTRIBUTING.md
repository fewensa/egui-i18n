# Contributing

Thanks for helping with egui-i18n. The library crate does not depend on egui.
Examples are native eframe 0.37 apps.

## Development

Install stable Rust with rustup. `rust-toolchain.toml` selects the stable
channel, rustfmt, clippy, and the `wasm32-unknown-unknown` target.

```
cargo test -p egui-i18n --features classic --test classic
cargo test -p egui-i18n --features fluent --test fluent
cargo clippy -p egui-i18n --features classic --all-targets -- -D warnings
cargo clippy -p egui-i18n --features fluent --all-targets -- -D warnings
cargo clippy -p egui-i18n-cli --all-targets -- -D warnings
cargo clippy -p egui-i18n-example-classic --all-targets -- -D warnings
cargo clippy -p egui-i18n-example-fluent --all-targets -- -D warnings
cargo fmt --all --check
cargo audit
cargo check -p egui-i18n --target wasm32-unknown-unknown --features classic
cargo check -p egui-i18n --target wasm32-unknown-unknown --no-default-features --features fluent
```

Run an example:

```
cargo run -p egui-i18n-example-classic
cargo run -p egui-i18n-example-fluent
```

The examples call `eframe::run_native` and do not build for
`wasm32-unknown-unknown`. The library does.

`classic` and `fluent` are mutually exclusive. Enabling `fluent` selects the
Fluent backend.

## Pull requests

- Keep the public library API source-compatible within the 0.2 line unless the
  change is intentionally breaking. Breaking changes need a minor version bump
  and a changelog entry.
- Format with `cargo fmt` before pushing. CI uses stable rustfmt.
- Add or update tests when behavior changes.
- Do not commit secrets. crates.io publishing uses the `CARGO_REGISTRY_TOKEN`
  GitHub Actions secret and is not run from pull requests.

## Security

Report vulnerabilities privately. See [SECURITY.md](SECURITY.md).
