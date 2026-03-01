//! Integration tests for the `fluent` translation backend.
//!
//! These tests exercise the public API surface of `egui-i18n` when compiled
//! with the `fluent` feature.  Each test uses a unique, valid BCP 47 language
//! tag to avoid cross-test interference when the suite runs in parallel, since
//! the translation registry is a process-wide static.
//!
//! Tests that read back through the global language/fallback state are run
//! serially via a shared `Mutex` guard to prevent data races.

#![cfg(feature = "fluent")]

use egui_i18n::fluent::FluentArgs;
use std::sync::Mutex;

// Guard used by tests that set / read the global language, fallback, or
// use_isolating state.
static SERIAL: Mutex<()> = Mutex::new(());

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn load(lang: &str, ftl: &str) {
  egui_i18n::load_translations_from_text(lang, ftl)
    .unwrap_or_else(|e| panic!("load_translations_from_text({lang}) failed: {e}"));
}

/// Load a bundle with `use_isolating = false` then restore the flag to `true`.
/// Must be called while holding `SERIAL` to avoid racing with other tests.
fn load_no_iso(lang: &str, ftl: &str) {
  egui_i18n::set_use_isolating(false);
  load(lang, ftl);
  egui_i18n::set_use_isolating(true);
}

// ---------------------------------------------------------------------------
// set_language / get_language / set_fallback / get_fallback
// ---------------------------------------------------------------------------

#[test]
fn test_set_get_language() {
  let _g = SERIAL.lock().unwrap();
  egui_i18n::set_language("en-US");
  assert_eq!(egui_i18n::get_language(), "en-US");
}

#[test]
fn test_set_get_fallback() {
  let _g = SERIAL.lock().unwrap();
  egui_i18n::set_fallback("fr-FR");
  assert_eq!(egui_i18n::get_fallback(), "fr-FR");
}

// ---------------------------------------------------------------------------
// set_use_isolating / get_use_isolating
// ---------------------------------------------------------------------------

#[test]
fn test_use_isolating_default_is_true() {
  let _g = SERIAL.lock().unwrap();
  egui_i18n::set_use_isolating(true);
  assert!(egui_i18n::get_use_isolating());
}

#[test]
fn test_set_use_isolating_false() {
  let _g = SERIAL.lock().unwrap();
  egui_i18n::set_use_isolating(false);
  assert!(!egui_i18n::get_use_isolating());
  egui_i18n::set_use_isolating(true);
}

// ---------------------------------------------------------------------------
// load_translations_from_map — must return Err in fluent mode
// ---------------------------------------------------------------------------

#[test]
fn test_load_from_map_returns_err() {
  let mut map = std::collections::HashMap::new();
  map.insert("key".to_string(), "value".to_string());
  let result = egui_i18n::load_translations_from_map("en-US", map);
  assert!(result.is_err(), "expected Err in fluent mode, got Ok");
  let msg = result.unwrap_err();
  assert!(msg.contains("fluent"), "error message should mention 'fluent': {msg}");
}

// ---------------------------------------------------------------------------
// load_translations_from_text
// ---------------------------------------------------------------------------

#[test]
fn test_load_valid_ftl() {
  load("en-AU", "greet-au = G'day!");
  assert!(egui_i18n::languages().contains(&"en-AU".to_string()));
}

#[test]
fn test_load_invalid_lang_id_returns_err() {
  // Empty string is not a valid BCP 47 identifier.
  let result = egui_i18n::load_translations_from_text("", "hello = Hello");
  assert!(result.is_err());
}

#[test]
fn test_load_duplicate_message_id_returns_err() {
  // fluent-bundle treats duplicate ids as an error.
  let ftl = "dup = one\ndup = two";
  let result = egui_i18n::load_translations_from_text("en-NZ", ftl);
  assert!(result.is_err());
}

#[test]
fn test_load_overwrites_previous_bundle() {
  let _g = SERIAL.lock().unwrap();
  load_no_iso("en-CA", "msg = old value");
  load_no_iso("en-CA", "msg = new value");
  egui_i18n::set_language("en-CA");
  egui_i18n::set_fallback("en-CA");
  let result = egui_i18n::translate_fluent("msg", &FluentArgs::new());
  assert_eq!(result, "new value");
}

// ---------------------------------------------------------------------------
// translate_fluent — basic
// ---------------------------------------------------------------------------

#[test]
fn test_translate_no_args() {
  let _g = SERIAL.lock().unwrap();
  load_no_iso("en-IE", "welcome-ie = Welcome!");
  egui_i18n::set_language("en-IE");
  egui_i18n::set_fallback("en-IE");
  assert_eq!(egui_i18n::translate_fluent("welcome-ie", &FluentArgs::new()), "Welcome!");
}

#[test]
fn test_translate_single_arg() {
  let _g = SERIAL.lock().unwrap();
  load_no_iso("en-SG", "hi-sg = Hi, { $name }!");
  egui_i18n::set_language("en-SG");
  egui_i18n::set_fallback("en-SG");
  let mut args = FluentArgs::new();
  args.set("name", "Wei");
  assert_eq!(egui_i18n::translate_fluent("hi-sg", &args), "Hi, Wei!");
}

#[test]
fn test_translate_multiple_args() {
  let _g = SERIAL.lock().unwrap();
  load_no_iso("en-ZA", "intro-za = I am { $name } and I am { $age } years old.");
  egui_i18n::set_language("en-ZA");
  egui_i18n::set_fallback("en-ZA");
  let mut args = FluentArgs::new();
  args.set("name", "Sipho");
  args.set("age", 28_i64);
  assert_eq!(egui_i18n::translate_fluent("intro-za", &args), "I am Sipho and I am 28 years old.");
}

#[test]
fn test_translate_missing_key_returns_empty() {
  let _g = SERIAL.lock().unwrap();
  load_no_iso("en-PH", "existing-ph = yes");
  egui_i18n::set_language("en-PH");
  egui_i18n::set_fallback("en-PH");
  assert_eq!(egui_i18n::translate_fluent("no-such-key", &FluentArgs::new()), "");
}

// ---------------------------------------------------------------------------
// translate_fluent — Unicode isolation marks
// ---------------------------------------------------------------------------

#[test]
fn test_isolating_enabled_wraps_placeable() {
  let _g = SERIAL.lock().unwrap();
  // Load with isolating ON (default).
  egui_i18n::set_use_isolating(true);
  load("de-AT", "greet-at = Hallo, { $name }!");
  egui_i18n::set_language("de-AT");
  egui_i18n::set_fallback("de-AT");
  let mut args = FluentArgs::new();
  args.set("name", "Hans");
  let result = egui_i18n::translate_fluent("greet-at", &args);
  // FSI (U+2068) and PDI (U+2069) must wrap the placeable.
  assert!(result.contains('\u{2068}'), "expected FSI mark in: {result:?}");
  assert!(result.contains('\u{2069}'), "expected PDI mark in: {result:?}");
  assert!(result.contains("Hans"));
}

#[test]
fn test_isolating_disabled_no_marks() {
  let _g = SERIAL.lock().unwrap();
  load_no_iso("de-CH", "greet-ch = Hallo, { $name }!");
  egui_i18n::set_language("de-CH");
  egui_i18n::set_fallback("de-CH");
  let mut args = FluentArgs::new();
  args.set("name", "Luca");
  let result = egui_i18n::translate_fluent("greet-ch", &args);
  assert!(!result.contains('\u{2068}'), "unexpected FSI mark in: {result:?}");
  assert!(!result.contains('\u{2069}'), "unexpected PDI mark in: {result:?}");
  assert_eq!(result, "Hallo, Luca!");
}

// ---------------------------------------------------------------------------
// translate_fluent — fallback
// ---------------------------------------------------------------------------

#[test]
fn test_translate_falls_back_when_primary_language_not_loaded() {
  let _g = SERIAL.lock().unwrap();
  load_no_iso("es-MX", "msg-mx = Hola");
  // "pt-BR" is never loaded; should fall back to "es-MX".
  egui_i18n::set_language("pt-BR");
  egui_i18n::set_fallback("es-MX");
  assert_eq!(egui_i18n::translate_fluent("msg-mx", &FluentArgs::new()), "Hola");
}

#[test]
fn test_translate_falls_back_when_key_absent_in_primary() {
  let _g = SERIAL.lock().unwrap();
  // "es-AR" has "shared-ar"; "pt-PT" does not.
  load_no_iso("es-AR", "shared-ar = Compartido");
  load_no_iso("pt-PT", "other-pt = Outro");
  egui_i18n::set_language("pt-PT");
  egui_i18n::set_fallback("es-AR");
  assert_eq!(egui_i18n::translate_fluent("shared-ar", &FluentArgs::new()), "Compartido");
}

#[test]
fn test_translate_primary_wins_over_fallback() {
  let _g = SERIAL.lock().unwrap();
  load_no_iso("nl-NL", "msg-nl = Primary Dutch");
  load_no_iso("nl-BE", "msg-nl = Fallback Belgian");
  egui_i18n::set_language("nl-NL");
  egui_i18n::set_fallback("nl-BE");
  assert_eq!(egui_i18n::translate_fluent("msg-nl", &FluentArgs::new()), "Primary Dutch");
}

#[test]
fn test_translate_uses_fallback_when_language_empty() {
  let _g = SERIAL.lock().unwrap();
  load_no_iso("sv-SE", "msg-se = Hej");
  egui_i18n::set_language("");
  egui_i18n::set_fallback("sv-SE");
  assert_eq!(egui_i18n::translate_fluent("msg-se", &FluentArgs::new()), "Hej");
}

#[test]
fn test_translate_returns_empty_when_both_langs_empty() {
  let _g = SERIAL.lock().unwrap();
  egui_i18n::set_language("");
  egui_i18n::set_fallback("");
  assert_eq!(egui_i18n::translate_fluent("any", &FluentArgs::new()), "");
}

// ---------------------------------------------------------------------------
// translate_fluent — Fluent-specific syntax
// ---------------------------------------------------------------------------

#[test]
fn test_plural_selector() {
  let _g = SERIAL.lock().unwrap();
  let ftl = "item-count = { $n ->\n    [one] One item\n   *[other] { $n } items\n}";
  load_no_iso("en-IN", ftl);
  egui_i18n::set_language("en-IN");
  egui_i18n::set_fallback("en-IN");

  let mut args_one = FluentArgs::new();
  args_one.set("n", 1_i64);
  assert_eq!(egui_i18n::translate_fluent("item-count", &args_one), "One item");

  let mut args_many = FluentArgs::new();
  args_many.set("n", 42_i64);
  assert_eq!(egui_i18n::translate_fluent("item-count", &args_many), "42 items");
}

#[test]
fn test_exact_number_selector() {
  let _g = SERIAL.lock().unwrap();
  let ftl = "score = { $n ->\n    [0] Zero!\n    [1] One!\n   *[other] { $n }\n}";
  load_no_iso("fi-FI", ftl);
  egui_i18n::set_language("fi-FI");
  egui_i18n::set_fallback("fi-FI");

  let mut args = FluentArgs::new();
  args.set("n", 0_i64);
  assert_eq!(egui_i18n::translate_fluent("score", &args), "Zero!");

  let mut args2 = FluentArgs::new();
  args2.set("n", 1_i64);
  assert_eq!(egui_i18n::translate_fluent("score", &args2), "One!");

  let mut args3 = FluentArgs::new();
  args3.set("n", 99_i64);
  assert_eq!(egui_i18n::translate_fluent("score", &args3), "99");
}

// ---------------------------------------------------------------------------
// tr! macro
// ---------------------------------------------------------------------------

#[test]
fn test_tr_macro_no_args() {
  let _g = SERIAL.lock().unwrap();
  load_no_iso("ko-KR", "title-kr = My App");
  egui_i18n::set_language("ko-KR");
  egui_i18n::set_fallback("ko-KR");
  assert_eq!(egui_i18n::tr!("title-kr"), "My App");
}

#[test]
fn test_tr_macro_with_single_arg() {
  let _g = SERIAL.lock().unwrap();
  load_no_iso("ko-KP", "greet-kp = Hello, { $name }!");
  egui_i18n::set_language("ko-KP");
  egui_i18n::set_fallback("ko-KP");
  assert_eq!(egui_i18n::tr!("greet-kp", { name: "World" }), "Hello, World!");
}

#[test]
fn test_tr_macro_with_multiple_args() {
  let _g = SERIAL.lock().unwrap();
  load_no_iso("zh-TW", "intro-tw = { $name } is { $age } years old.");
  egui_i18n::set_language("zh-TW");
  egui_i18n::set_fallback("zh-TW");
  assert_eq!(egui_i18n::tr!("intro-tw", { name: "Ada", age: 32_i64 }), "Ada is 32 years old.");
}

// ---------------------------------------------------------------------------
// languages
// ---------------------------------------------------------------------------

#[test]
fn test_languages_contains_loaded() {
  load_no_iso("hu-HU", "k = v");
  assert!(egui_i18n::languages().contains(&"hu-HU".to_string()));
}

#[test]
fn test_languages_does_not_contain_unloaded() {
  assert!(!egui_i18n::languages().contains(&"x-fl-never-loaded".to_string()));
}
