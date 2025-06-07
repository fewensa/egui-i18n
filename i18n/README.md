# egui-i18n

**`egui-i18n`** is an internationalization (i18n) solution designed specifically for the [egui](https://github.com/emilk/egui) framework. It supports both [Fluent](https://projectfluent.org/) syntax and traditional key-value translation formats. With flexible resource loading, dynamic parameter interpolation, and performance optimizations, it helps developers easily implement multi-language support in their applications.

---

## 🛠 Project Origin

`egui-i18n` originated from a [proposal to add i18n support](https://github.com/emilk/egui/pull/5403) in the official `egui` repository. Since `egui` does not currently include a built-in i18n system, this project was developed independently to fulfill real-world needs without modifying `egui`'s core. The goals include:

- Provide multi-language support **without modifying egui's source code**.
- Support both **Fluent** and **classic key-value** translation formats.
- Enable **runtime language switching and dynamic parameter interpolation**.
- Deliver high performance and flexibility for **Rust-based GUI applications**.

---

## ✨ Features

### 🗣 Multi-language Support

- **[Fluent](https://projectfluent.org/)**: Ideal for complex linguistic structures.
- **Classic key-value format**: Suitable for simple and straightforward translations.

### 🔄 Dynamic Parameter Interpolation

- Easily insert dynamic values (e.g. names, dates, numbers) into translation strings.

### 📂 Flexible Resource Loading

- Load `.ftl` (Fluent) or `.properties` (key-value) files from specified file paths.
- Language fallback support (e.g. fallback from `zh-HK` to `zh`).

### ⚡ High Performance

- Built-in caching system to speed up parsing and lookup of translation resources, suitable for real-time UI rendering.

---

## 📦 Installation

Add the dependency in your `Cargo.toml`:

```toml
[dependencies]
egui-i18n = "0.1"
```

---

## 🚀 Getting Started

### Using Fluent Translations

```toml
[dependencies]
egui-i18n = { version = "0.1", features = ["fluent"] }
```

```rust
use egui_i18n::tr;

fn main() {
  let greeting = tr!("greeting", { name: "Alice" });
  println!("{}", greeting); // Output: Hello, Alice!
}
```

Fluent resource file example (`en-US.ftl`):

```
greeting = Hello, { $name }!
```

---

### Using Classic Key-Value Translations

```rust
use egui_i18n::tr;

fn main() {
  let greeting = tr!("classic_greeting");
  println!("{}", greeting); // Output: Hello, world!
}
```

Classic key-value resource example:

```
classic_greeting = Hello, world!
```

---

## ⚙️ Configuration Options

### Cargo Features

- `fluent`: Enables Fluent translation mode.
- `classic`: Enables classic key-value translation mode (enabled by default).

---

## 📄 License

This project is open source under the [MIT License](https://opensource.org/licenses/MIT). Contributions and feedback are welcome!

---

For more examples, API documentation, or integration guides, check the project source code and the [`examples`](https://github.com/fewensa/egui-i18n/tree/main/examples) directory. If you encounter any issues or have suggestions, feel free to open an issue or submit a PR!
