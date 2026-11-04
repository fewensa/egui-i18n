# egui-i18n-cli

Command-line helper for [egui-i18n](https://github.com/fewensa/egui-i18n). It scans Rust
sources for `tr!` calls and appends any new keys to translation files.

```
egui-i18n-cli generate \
    --source-path ./src \
    --output-path ./i18n \
    --language en-US \
    --language zh-Hans \
    --default-language en-US \
    --ext egl
```

See the [project documentation](https://github.com/fewensa/egui-i18n/blob/main/i18n/README.md)
for the full option list. The library crate is `egui-i18n`; this binary is `egui-i18n-cli`.
