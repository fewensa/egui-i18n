## Summary

<!-- What changed and why. -->

## Test plan

- [ ] `cargo test -p egui-i18n --features classic --test classic`
- [ ] `cargo test -p egui-i18n --features fluent --test fluent`
- [ ] `cargo clippy --all-targets -- -D warnings` for the crates this PR touches
- [ ] `cargo fmt --all --check`
- [ ] `cargo audit`
