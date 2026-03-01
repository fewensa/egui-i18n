# egui-i18n

An internationalization (i18n) library for [egui](https://github.com/emilk/egui), providing
runtime language switching, dynamic parameter interpolation, and a language fallback chain —
without any modifications to egui's source code.

Supports two independent translation backends selectable via Cargo features:

| Feature | Backend | File format |
|---------|---------|-------------|
| `classic` *(default)* | Key-value pairs | `.egl` / `.properties` |
| `fluent` | [Mozilla Fluent](https://projectfluent.org/) | `.ftl` |

---

## Installation

```toml
[dependencies]
# Classic key-value backend (default)
egui-i18n = "0.1"

# Fluent backend
egui-i18n = { version = "0.1", features = ["fluent"] }
```

> The two features are mutually exclusive. Enabling `fluent` disables `classic`.

---

## Quick Start

### Classic backend

**Translation file** (`assets/en-US.egl`):

```
# Lines beginning with # are comments and are ignored.
welcome = Welcome to the app!
hello-name = Hello, {name}!
intro = My name is {name} and I am {age} years old.

# Keys that contain a literal = must escape it with \=
Hello\=, {name}! = Hello=, {name}!
```

**Rust code**:

```rust
use egui_i18n::tr;

fn init() {
    let en = include_str!("../assets/en-US.egl");
    let zh = include_str!("../assets/zh-CN.egl");

    egui_i18n::load_translations_from_text("en-US", en).unwrap();
    egui_i18n::load_translations_from_text("zh-CN", zh).unwrap();

    egui_i18n::set_language("en-US");
    egui_i18n::set_fallback("en-US");
}

// Inside your egui update loop:
ui.label(tr!("welcome"));
ui.label(tr!("hello-name", { name: &self.name }));
ui.label(tr!("intro", { name: &self.name, age: self.age }));
```

---

### Fluent backend

**Translation file** (`assets/en-US.ftl`):

```fluent
welcome = Welcome to the app!
hello-name = Hello, { $name }!
item-count =
    { $count ->
        [one] One item
       *[other] { $count } items
    }
```

**Rust code**:

```rust
use egui_i18n::tr;

fn init() {
    // On Windows, Fluent wraps placeables in Unicode directionality marks
    // (U+2068 / U+2069) that some native text renderers display as garbage.
    // Disable them before loading any bundles.
    #[cfg(target_os = "windows")]
    egui_i18n::set_use_isolating(false);

    let en = include_str!("../assets/en-US.ftl");
    let zh = include_str!("../assets/zh-Hans.ftl");

    egui_i18n::load_translations_from_text("en-US", en).unwrap();
    egui_i18n::load_translations_from_text("zh-Hans", zh).unwrap();

    egui_i18n::set_language("en-US");
    egui_i18n::set_fallback("en-US");
}

// Inside your egui update loop:
ui.label(tr!("welcome"));
ui.label(tr!("hello-name", { name: &self.name }));
ui.label(tr!("item-count", { count: self.count }));
```

---

## API Reference

### Language configuration

```rust
// Set the active display language.
egui_i18n::set_language("zh-Hans");

// Get the currently active language identifier.
let lang: String = egui_i18n::get_language();

// Set the fallback language used when a key is missing in the active language.
egui_i18n::set_fallback("en-US");

// Get the current fallback language identifier.
let fallback: String = egui_i18n::get_fallback();

// Return all language identifiers that have been loaded.
let langs: Vec<String> = egui_i18n::languages();
```

### Loading translations

```rust
// Load from an in-memory string (the most common approach with include_str!).
egui_i18n::load_translations_from_text("en-US", content)?;

// Load from a HashMap<String, String> — classic backend only.
// Returns Err in fluent mode.
egui_i18n::load_translations_from_map("en-US", map)?;

// Scan a directory and load every .egl / .ftl file found.
// Each file's stem is used as the language identifier.
egui_i18n::load_translations_from_path("/path/to/i18n/")?;
```

### Translating

```rust
// No arguments.
let s: String = tr!("welcome");

// One or more named arguments.
let s: String = tr!("hello-name", { name: &self.name });
let s: String = tr!("intro", { name: &self.name, age: self.age });
```

### Fluent-only options

```rust
// Control whether Fluent wraps placeables in Unicode directionality marks.
// Default: true. Set to false before loading bundles to suppress the marks.
egui_i18n::set_use_isolating(false);

// Query the current setting.
let isolating: bool = egui_i18n::get_use_isolating();
```

---

## Translation file format

### Classic (`.egl`)

```
# Comment — this line is ignored entirely.
key = value
hello-name = Hello, {name}!

# Multi-line values: lines without = are appended to the previous key.
long-text = Line one
continues here
still the same value

# Keys containing a literal = must escape it with \=
A\=B = A equals B
```

Rules:
- Lines starting with `#` are comments.
- Both Unix (`LF`) and Windows (`CRLF`) line endings are supported.
- Leading and trailing whitespace around keys and values is trimmed.
- Placeholders use `{name}` syntax and are replaced at runtime.

### Fluent (`.ftl`)

Fluent is a fully-featured localization system. See the
[Fluent syntax guide](https://projectfluent.org/fluent/guide/) for the complete reference.
Key capabilities used here include:

- Named arguments: `{ $name }`
- Plural selectors: `{ $count -> [one] … *[other] … }`
- Exact number matching: `{ $n -> [0] Zero [1] One *[other] … }`

---

## Fallback behaviour

When a translation key is looked up:

1. The active language (`set_language`) is searched first.
2. If the key is not found (or the language was never loaded), the fallback language
   (`set_fallback`) is tried.
3. If neither contains the key, an empty string is returned.

```rust
egui_i18n::set_language("ja-JP");   // primary
egui_i18n::set_fallback("en-US");   // used when primary is missing a key
```

> **Important**: call all configuration functions (`set_language`, `set_fallback`,
> `set_use_isolating`) *before* loading any translation bundles, so that each bundle
> is constructed with the correct settings.

---

## Loading from the filesystem

`load_translations_from_path` accepts either a file path or a directory path.

```rust
// Load a single file — its stem becomes the language identifier.
egui_i18n::load_translations_from_path("i18n/en-US.ftl")?;

// Load all .egl / .ftl files in a directory.
egui_i18n::load_translations_from_path("i18n/")?;
// This is equivalent to loading i18n/en-US.ftl → "en-US",
//                                  i18n/zh-Hans.ftl → "zh-Hans", etc.
```

---

## CLI tool

The companion `egui-i18n-cli` tool scans Rust source files for `tr!(...)` macro calls,
extracts the translation keys, and generates or updates translation files with stubs for
any newly discovered keys.

```
egui-i18n-cli generate \
    --source-path ./src \
    --output-path ./i18n \
    --language en-US \
    --language zh-Hans \
    --default-language en-US \
    --ext egl
```

| Option | Description |
|--------|-------------|
| `--source-path` | Root directory to scan for `.rs` files |
| `--output-path` | Directory where translation files are written (default: current directory) |
| `--extension` | Additional file extensions to scan (default: `rs`) |
| `--language` | Language identifiers to generate files for (default: `en_US`) |
| `--default-language` | When set, newly found keys in this language are pre-filled with the key itself as the value |
| `--ext` | Output file extension: `egl` or `ftl` (default: `tgl`) |

Running the command again is safe — existing translations are preserved and only new keys
are appended.

---

## Examples

Two fully runnable example applications are included:

| Example | Command |
|---------|---------|
| Classic backend | `cargo run -p egui-i18n-example-classic` |
| Fluent backend | `cargo run -p egui-i18n-example-fluent` |

Both examples demonstrate runtime language switching via on-screen buttons and
dynamic argument interpolation.

---

## Feature flags

| Feature | Description | Default |
|---------|-------------|---------|
| `classic` | Enable the key-value translation backend | ✅ yes |
| `fluent` | Enable the Mozilla Fluent translation backend | ❌ no |

The two features are mutually exclusive. If `fluent` is enabled, `classic` is
automatically disabled.

---

## License

This project is licensed under the [MIT License](LICENSE).