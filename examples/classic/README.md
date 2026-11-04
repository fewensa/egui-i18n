# egui-i18n classic example

Native [eframe 0.37](https://crates.io/crates/eframe) app using the default classic
key-value backend.

```
cargo run -p egui-i18n-example-classic
```

Translation files live in `assets/`. This example targets native egui. It does not
build for `wasm32-unknown-unknown`, because it calls `eframe::run_native`.
The `egui-i18n` library itself does build for that target.
