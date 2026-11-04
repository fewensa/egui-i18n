# Changelog

All notable changes to this project are documented in this file.

The `egui-i18n` library does not depend on egui. Versions below describe the
library API and the egui version the examples are written for.

## 0.2.1 - 2026-11-04

### Changed

- Examples use eframe, egui, and egui_extras 0.37 (previously 0.31). eframe 0.34
  replaced `App::update(&mut self, &Context, &mut Frame)` with
  `App::ui(&mut self, &mut Ui, &mut Frame)`. Both examples use `App::ui`.
- Direct dependencies are updated to their latest compatible releases, including
  `syn` 3 and `clap` 4.
- `egui-i18n-cli` now uses `clap` instead of unmaintained `structopt` and `paw`.
  The `generate` subcommand and its flags are unchanged.
- Example translation files and the Ferris image live inside each example crate
  so the packages build on their own.

### Security

- Dropped `structopt` (RUSTSEC-2022-0104, unmaintained) and `paw`.
- `cargo audit` reports no vulnerabilities. Four informational "unmaintained"
  advisories remain only in the eframe example tree (`ttf-parser` via winit,
  and `zvariant` / `zvariant_derive` / `zbus_names` via accesskit). None have a
  patched release that eframe 0.37 can use. They are recorded in
  `.cargo/audit.toml`. The `egui-i18n` library does not depend on those crates.

### Compatibility

- Library API (`tr!`, language loading, fallback, Fluent isolating marks) is
  source-compatible with 0.2.0.
- Library MSRV stays Rust 1.77.
- Building the examples requires Rust 1.95, which is eframe 0.37's MSRV.
- The CLI MSRV is Rust 1.85 (clap 4.6).

## 0.2.0

- Selectable classic and Fluent backends, integration tests, and an option to
  disable Fluent isolating marks.

## 0.1.1

- Dependency and packaging fixes on top of the first release.

## 0.1.0

- Initial release.
