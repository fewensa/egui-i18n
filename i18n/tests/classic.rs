//! Integration tests for the `classic` (default) translation backend.
//!
//! These tests exercise the public API surface of `egui-i18n` when compiled
//! without the `fluent` feature.  Each test uses a unique language tag to
//! avoid cross-test interference when the suite runs in parallel, since the
//! translation registry is a process-wide static.
//!
//! Tests that read back through the global language/fallback state are run
//! serially via a shared `Mutex` guard to prevent data races.

#![cfg(not(feature = "fluent"))]

use std::collections::HashMap;
use std::sync::Mutex;

// Guard used by tests that set / read the global language and fallback state.
static SERIAL: Mutex<()> = Mutex::new(());

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn load(lang: &str, content: &str) {
  egui_i18n::load_translations_from_text(lang, content)
    .expect("load_translations_from_text failed");
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
// load_translations_from_text
// ---------------------------------------------------------------------------

#[test]
fn test_load_from_text_simple() {
  load("cl-int-load-simple", "hello = Hello");
  assert!(egui_i18n::languages().contains(&"cl-int-load-simple".to_string()));
}

#[test]
fn test_load_from_text_multiple_keys() {
  load("cl-int-load-multi", "a = A\nb = B\nc = C");
  assert!(egui_i18n::languages().contains(&"cl-int-load-multi".to_string()));
}

#[test]
fn test_load_from_text_overwrites_previous() {
  let _g = SERIAL.lock().unwrap();
  load("cl-int-overwrite", "msg = old");
  load("cl-int-overwrite", "msg = new");
  egui_i18n::set_language("cl-int-overwrite");
  egui_i18n::set_fallback("cl-int-overwrite");
  let result = egui_i18n::translate_classic("msg", &HashMap::new());
  assert_eq!(result, "new");
}

// ---------------------------------------------------------------------------
// load_translations_from_map
// ---------------------------------------------------------------------------

#[test]
fn test_load_from_map_ok() {
  let mut map = HashMap::new();
  map.insert("key".to_string(), "value".to_string());
  assert!(egui_i18n::load_translations_from_map("cl-int-map-ok", map).is_ok());
  assert!(egui_i18n::languages().contains(&"cl-int-map-ok".to_string()));
}

#[test]
fn test_load_from_map_translates_correctly() {
  let _g = SERIAL.lock().unwrap();
  let mut map = HashMap::new();
  map.insert("greeting".to_string(), "Hello, {name}!".to_string());
  egui_i18n::load_translations_from_map("cl-int-map-tr", map).unwrap();
  egui_i18n::set_language("cl-int-map-tr");
  egui_i18n::set_fallback("cl-int-map-tr");
  let mut args = HashMap::new();
  args.insert("name", "Alice".to_string());
  assert_eq!(egui_i18n::translate_classic("greeting", &args), "Hello, Alice!");
}

// ---------------------------------------------------------------------------
// translate_classic — basic
// ---------------------------------------------------------------------------

#[test]
fn test_translate_no_args() {
  let _g = SERIAL.lock().unwrap();
  load("cl-int-no-args", "welcome = Welcome!");
  egui_i18n::set_language("cl-int-no-args");
  egui_i18n::set_fallback("cl-int-no-args");
  assert_eq!(egui_i18n::translate_classic("welcome", &HashMap::new()), "Welcome!");
}

#[test]
fn test_translate_single_arg() {
  let _g = SERIAL.lock().unwrap();
  load("cl-int-single-arg", "hi = Hi, {name}!");
  egui_i18n::set_language("cl-int-single-arg");
  egui_i18n::set_fallback("cl-int-single-arg");
  let mut args = HashMap::new();
  args.insert("name", "Bob".to_string());
  assert_eq!(egui_i18n::translate_classic("hi", &args), "Hi, Bob!");
}

#[test]
fn test_translate_multiple_args() {
  let _g = SERIAL.lock().unwrap();
  load("cl-int-multi-arg", "intro = I am {name} and I am {age} years old.");
  egui_i18n::set_language("cl-int-multi-arg");
  egui_i18n::set_fallback("cl-int-multi-arg");
  let mut args = HashMap::new();
  args.insert("name", "Carol".to_string());
  args.insert("age", "30".to_string());
  assert_eq!(egui_i18n::translate_classic("intro", &args), "I am Carol and I am 30 years old.");
}

#[test]
fn test_translate_missing_key_returns_empty() {
  let _g = SERIAL.lock().unwrap();
  load("cl-int-miss-key", "existing = yes");
  egui_i18n::set_language("cl-int-miss-key");
  egui_i18n::set_fallback("cl-int-miss-key");
  assert_eq!(egui_i18n::translate_classic("no-such-key", &HashMap::new()), "");
}

#[test]
fn test_translate_unknown_placeholder_left_intact() {
  let _g = SERIAL.lock().unwrap();
  load("cl-int-unknown-ph", "msg = Hello, {unknown}!");
  egui_i18n::set_language("cl-int-unknown-ph");
  egui_i18n::set_fallback("cl-int-unknown-ph");
  assert_eq!(egui_i18n::translate_classic("msg", &HashMap::new()), "Hello, {unknown}!");
}

// ---------------------------------------------------------------------------
// translate_classic — fallback
// ---------------------------------------------------------------------------

#[test]
fn test_translate_falls_back_when_primary_missing() {
  let _g = SERIAL.lock().unwrap();
  load("cl-int-fb-primary", "msg = from primary");
  load("cl-int-fb-fallback", "msg = from fallback");
  load("cl-int-fb-fallback", "fallback-only = only here");
  egui_i18n::set_language("cl-int-fb-primary");
  egui_i18n::set_fallback("cl-int-fb-fallback");
  assert_eq!(egui_i18n::translate_classic("fallback-only", &HashMap::new()), "only here");
}

#[test]
fn test_translate_primary_wins_over_fallback() {
  let _g = SERIAL.lock().unwrap();
  load("cl-int-primary-wins", "msg = primary value");
  load("cl-int-primary-wins-fb", "msg = fallback value");
  egui_i18n::set_language("cl-int-primary-wins");
  egui_i18n::set_fallback("cl-int-primary-wins-fb");
  assert_eq!(egui_i18n::translate_classic("msg", &HashMap::new()), "primary value");
}

#[test]
fn test_translate_uses_fallback_when_language_empty() {
  let _g = SERIAL.lock().unwrap();
  load("cl-int-empty-primary", "msg = fallback result");
  egui_i18n::set_language("");
  egui_i18n::set_fallback("cl-int-empty-primary");
  assert_eq!(egui_i18n::translate_classic("msg", &HashMap::new()), "fallback result");
}

#[test]
fn test_translate_returns_empty_when_both_langs_empty() {
  let _g = SERIAL.lock().unwrap();
  egui_i18n::set_language("");
  egui_i18n::set_fallback("");
  assert_eq!(egui_i18n::translate_classic("any", &HashMap::new()), "");
}

// ---------------------------------------------------------------------------
// parse_translations
// ---------------------------------------------------------------------------

#[test]
fn test_parse_simple_pairs() {
  let map = egui_i18n::parse_translations("hello = Hello\nworld = World", true);
  assert_eq!(map.get("hello").map(String::as_str), Some("Hello"));
  assert_eq!(map.get("world").map(String::as_str), Some("World"));
}

#[test]
fn test_parse_comment_lines_skipped() {
  let map = egui_i18n::parse_translations("# comment\nhello = Hello", true);
  assert!(!map.contains_key("# comment"));
  assert_eq!(map.get("hello").map(String::as_str), Some("Hello"));
}

#[test]
fn test_parse_crlf_line_endings() {
  let map = egui_i18n::parse_translations("a = foo\r\nb = bar", true);
  assert_eq!(map.get("a").map(String::as_str), Some("foo"));
  assert_eq!(map.get("b").map(String::as_str), Some("bar"));
}

#[test]
fn test_parse_escaped_equals_in_key() {
  let map = egui_i18n::parse_translations("Hello\\=, {name}! = Hello=, {name}!", true);
  assert_eq!(map.get("Hello=, {name}!").map(String::as_str), Some("Hello=, {name}!"));
}

#[test]
fn test_parse_multiline_value() {
  let content = "msg = line one\ncontinuation\nend = done";
  let map = egui_i18n::parse_translations(content, true);
  assert_eq!(map.get("msg").map(String::as_str), Some("line one\ncontinuation"));
  assert_eq!(map.get("end").map(String::as_str), Some("done"));
}

#[test]
fn test_parse_clean_empty_true_drops_empty_values() {
  let map = egui_i18n::parse_translations("present = hello\nempty =", true);
  assert!(map.contains_key("present"));
  assert!(!map.contains_key("empty"));
}

#[test]
fn test_parse_clean_empty_false_keeps_empty_values() {
  let map = egui_i18n::parse_translations("present = hello\nempty =", false);
  assert!(map.contains_key("present"));
  assert!(map.contains_key("empty"));
}

#[test]
fn test_parse_trims_whitespace() {
  let map = egui_i18n::parse_translations("  key  =  value  ", true);
  assert_eq!(map.get("key").map(String::as_str), Some("value"));
}

#[test]
fn test_parse_empty_content() {
  assert!(egui_i18n::parse_translations("", true).is_empty());
}

#[test]
fn test_parse_only_comments() {
  assert!(egui_i18n::parse_translations("# one\n# two", true).is_empty());
}

// ---------------------------------------------------------------------------
// tr! macro
// ---------------------------------------------------------------------------

#[test]
fn test_tr_macro_no_args() {
  let _g = SERIAL.lock().unwrap();
  load("cl-int-tr-macro-noarg", "title = My App");
  egui_i18n::set_language("cl-int-tr-macro-noarg");
  egui_i18n::set_fallback("cl-int-tr-macro-noarg");
  assert_eq!(egui_i18n::tr!("title"), "My App");
}

#[test]
fn test_tr_macro_with_args() {
  let _g = SERIAL.lock().unwrap();
  load("cl-int-tr-macro-arg", "greet = Hello, {name}!");
  egui_i18n::set_language("cl-int-tr-macro-arg");
  egui_i18n::set_fallback("cl-int-tr-macro-arg");
  assert_eq!(egui_i18n::tr!("greet", { name: "Alice" }), "Hello, Alice!");
}

// ---------------------------------------------------------------------------
// languages
// ---------------------------------------------------------------------------

#[test]
fn test_languages_contains_loaded() {
  load("cl-int-lang-list", "k = v");
  assert!(egui_i18n::languages().contains(&"cl-int-lang-list".to_string()));
}

#[test]
fn test_languages_does_not_contain_unloaded() {
  assert!(!egui_i18n::languages().contains(&"x-cl-never-loaded".to_string()));
}

