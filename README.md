# egui-i18n

An internationalization (i18n) library for [egui](https://github.com/emilk/egui), providing
runtime language switching, dynamic parameter interpolation, and a language fallback chain —
without any modifications to egui's source code.

Supports two independent translation backends selectable via Cargo features:

| Feature | Backend | File format |
|---------|---------|-------------|
| `classic` *(default)* | Key-value pairs | `.egl` / `.properties` |
| `fluent` | [Mozilla Fluent](https://projectfluent.org/) | `.ftl` |

## Compatibility

| Piece | Version |
|-------|---------|
| `egui-i18n` (this release) | 0.2.2 |
| egui / eframe / egui_extras (examples) | 0.37 |
| Rust, library only | 1.77+ |
| Rust, examples and full workspace | 1.95+ |

The library crate does not link egui, so the translation API works with any egui
version. The runnable examples, and any app code copied from them, target
eframe 0.37, whose `App::ui` method replaced `App::update`.

---

## Installation

```toml
[dependencies]
# Classic key-value backend (default)
egui-i18n = "0.2.2"

# Fluent backend
egui-i18n = { version = "0.2.2", features = ["fluent"] }
```

> The two features are mutually exclusive. Enabling `fluent` disables `classic`.

---

## Examples

Two fully runnable example applications are included:

| Example | Command |
|---------|---------|
| Classic backend | `cargo run -p egui-i18n-example-classic` |
| Fluent backend  | `cargo run -p egui-i18n-example-fluent`  |

Both examples demonstrate runtime language switching via on-screen buttons and dynamic
argument interpolation.

---

## Documentation

Full API reference, translation file format details, CLI tool usage, and integration
guides are in [i18n/README.md](./i18n/README.md).

---

## Project layout

```
egui-i18n/
├── i18n/          # Library crate (egui-i18n)
├── cli/           # CLI tool (egui-i18n-cli)
└── examples/
    ├── classic/   # Example using the classic key-value backend
    └── fluent/    # Example using the Fluent backend
```

---

## License

This project is licensed under the [MIT License](LICENSE).

See [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and
[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Release notes are in
[CHANGELOG.md](CHANGELOG.md).