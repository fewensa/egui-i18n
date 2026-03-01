use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

// When the `fluent` feature is active, the classic runtime (TRANSLATIONS,
// load_*, translate, etc.) is not used — only parse_translations is exported
// for the CLI tool.  Suppress the resulting dead_code warnings.
#[cfg_attr(feature = "fluent", allow(dead_code))]
static TRANSLATIONS: Lazy<RwLock<HashMap<String, HashMap<String, String>>>> =
  Lazy::new(|| RwLock::new(HashMap::new()));

#[cfg_attr(feature = "fluent", allow(dead_code))]
pub fn load_translations_from_text(
  language: impl AsRef<str>,
  content: impl AsRef<str>,
) -> Result<(), String> {
  let translations = parse_translations(content.as_ref(), true);
  load_translations_from_map(language, translations);
  Ok(())
}

#[cfg_attr(feature = "fluent", allow(dead_code))]
pub fn load_translations_from_map(
  language: impl AsRef<str>,
  translations: HashMap<String, String>,
) {
  let mut translations_map = TRANSLATIONS.write().unwrap();
  translations_map.insert(language.as_ref().to_string(), translations);
}

/// Parse a `.egl` / `.properties`-style translation file into a key-value map.
///
/// Format rules:
/// - Lines containing `=` are treated as `key = value` pairs.
/// - Keys that contain a literal `=` must escape it as `\=`.
/// - Lines that do not contain `=` are appended to the current value (multi-line
///   values).
/// - Lines beginning with `#` are treated as comments and ignored entirely.
/// - Both Unix (`\n`) and Windows (`\r\n`) line endings are accepted.
/// - When `clean_empty` is `true`, entries whose value is empty after trimming
///   are discarded (useful at runtime). When `false` they are kept (useful for
///   the CLI generator which needs to know all declared keys).
pub fn parse_translations(content: impl AsRef<str>, clean_empty: bool) -> HashMap<String, String> {
  let mut map = HashMap::new();
  let mut current_key = String::new();
  let mut value_lines: Vec<String> = Vec::new();

  let flush = |map: &mut HashMap<String, String>, key: &str, lines: &[String]| {
    if key.is_empty() {
      return;
    }
    let value = lines.join("\n").trim().to_string();
    if !value.is_empty() || !clean_empty {
      map.insert(key.to_string(), value);
    }
  };

  for raw_line in content.as_ref().lines() {
    // `str::lines` already handles both \n and \r\n, so no manual stripping needed.
    let line = raw_line;

    // Skip comment lines.
    if line.trim_start().starts_with('#') {
      continue;
    }

    if !line.contains('=') {
      // Continuation line for the current key's value.
      value_lines.push(line.to_string());
      continue;
    }

    // Commit the previous key before starting a new one.
    flush(&mut map, &current_key, &value_lines);
    value_lines.clear();

    if line.contains("\\=") {
      // The key contains one or more literal `=` characters escaped as `\=`.
      // Strategy: split on `\=` first, then find the first segment that
      // contains a plain `=` — that segment splits into the last part of the
      // key and the start of the value.
      let escaped_parts: Vec<&str> = line.split("\\=").collect();
      let mut key_parts: Vec<String> = Vec::new();
      let mut value_parts: Vec<String> = Vec::new();
      let mut found_value = false;

      for (i, part) in escaped_parts.iter().enumerate() {
        if found_value {
          // Everything after the first plain `=` is part of the value;
          // re-join with `\=` since these segments belong to the value side.
          value_parts.push(format!("\\={}", part));
          continue;
        }
        if part.contains('=') {
          let (k, v) = part.split_once('=').unwrap();
          key_parts.push(k.trim().to_string());
          value_parts.push(v.trim().to_string());
          found_value = true;
          // Any remaining escaped_parts beyond this index still belong to
          // the value (they were separated by `\=` which is part of the value).
          let _ = i; // suppress unused warning
        } else {
          key_parts.push(part.trim().to_string());
        }
      }

      current_key = key_parts.join("=");
      value_lines.push(value_parts.join(""));
    } else {
      let (k, v) = line.split_once('=').unwrap();
      current_key = k.trim().to_string();
      value_lines.push(v.trim().to_string());
    }
  }

  // Commit the last key.
  flush(&mut map, &current_key, &value_lines);

  map
}

#[cfg_attr(feature = "fluent", allow(dead_code))]
pub fn format(template: &str, args: &HashMap<&str, String>) -> String {
  let mut result = template.to_string();
  for (key, value) in args {
    result = result.replace(&format!("{{{}}}", key), value);
  }
  result
}

#[cfg_attr(feature = "fluent", allow(dead_code))]
pub fn translate(
  language: impl AsRef<str>,
  fallback_language: impl AsRef<str>,
  key: &str,
  args: &HashMap<&str, String>,
) -> String {
  let language = language.as_ref();
  let fallback_language = fallback_language.as_ref();
  if language.is_empty() && fallback_language.is_empty() {
    return String::default();
  }
  let language = if language.is_empty() { fallback_language } else { language };

  let mut translated = extract_translate(language, key, args);
  if translated.is_empty() {
    translated = extract_translate(fallback_language, key, args);
  }
  translated
}

#[cfg_attr(feature = "fluent", allow(dead_code))]
fn extract_translate(language: impl AsRef<str>, key: &str, args: &HashMap<&str, String>) -> String {
  let translations = TRANSLATIONS.read().unwrap();
  if let Some(language_map) = translations.get(language.as_ref()) {
    if let Some(template) = language_map.get(key) {
      if !template.is_empty() {
        return format(template, args);
      }
    }
  }
  String::default()
}

#[cfg_attr(feature = "fluent", allow(dead_code))]
pub fn languages() -> Vec<String> {
  let translations = TRANSLATIONS.read().unwrap();
  translations.keys().cloned().collect()
}
