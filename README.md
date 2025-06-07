# egui-i18n

**`egui-i18n`** is an internationalization (i18n) library for [egui](https://github.com/emilk/egui), offering seamless multi-language support. It supports both [Fluent](https://projectfluent.org/) and traditional key-value translation formats, with features like dynamic parameter interpolation, language fallback, and high-performance resource loading — ideal for Rust-based GUI applications.

---

## 🚀 Quick Start

Check out the example projects to see how to use `egui-i18n` in practice:

- 📄 [`classic`](./examples/classic/) – Example using the classic key-value format.
- 📄 [`fluent`](./examples/fluent/) – Example using Fluent's advanced syntax.

---

## 📚 Documentation Overview (See [i18n/README.md](./i18n/README.md) for full details)

The documentation covers the following:

- **Features**

  - Support for both Fluent and classic key-value translation formats.
  - Dynamic parameter interpolation (e.g., names, numbers).
  - Flexible resource loading (with language fallback and caching).
  - Optimized for real-time UI performance.

- **Installation**

  - How to add `egui-i18n` to your project.

- **Usage**

  - How to load `.ftl` or `.properties` translation files.
  - How to use the `tr!` macro for translation.

- **Configuration**

  - Cargo features supported: `fluent` / `classic`.

- **Translation Resource Examples**

  - Sample formats for both Fluent and key-value resources.

- **Dependencies & Integration**

  - Core dependencies and optional crates explained.

- **Contribution Guide**

  - How to contribute code, file issues, or submit PRs.

📖 **Read the full guide**: [i18n/README.md](./i18n/README.md)

---

## 📄 License

This project is open-sourced under the [MIT License](LICENSE). Contributions are welcome!
