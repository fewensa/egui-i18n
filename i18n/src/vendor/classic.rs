use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

static TRANSLATIONS: Lazy<RwLock<HashMap<String, HashMap<String, String>>>> =
  Lazy::new(|| RwLock::new(HashMap::new()));

pub fn load_translations_from_text(language: impl AsRef<str>, content: impl AsRef<str>) -> Result<(), String> {
  let translations = parse_translations(content.as_ref().to_string(), true);
  load_translations_from_map(language, translations);
  Ok(())
}

pub fn load_translations_from_map(
  language: impl AsRef<str>,
  translations: HashMap<String, String>,
) {
  let mut translations_map = TRANSLATIONS.write().unwrap();
  translations_map.insert(language.as_ref().to_string(), translations);
}

pub fn parse_translations(content: String, clean_empty: bool) -> HashMap<String, String> {
  let mut map = HashMap::new();
  let mut name = String::default();
  let mut values = vec![];
  for line in content.split("\n") {
    if !line.contains("=") {
      values.push(line.to_string());
      continue;
    }
    if !name.is_empty() {
      let value = values.join("\n").trim().to_string();
      let allow = if value.is_empty() { !clean_empty } else { true };
      if allow {
        map.insert(name, value);
      }
      // name = String::default();
      values.clear();
    }
    if line.contains("\\=") {
      let items_of_escaping: Vec<&str> = line.split("\\=").collect();
      let mut e_names = vec![];
      let mut e_values = vec![];
      let len = items_of_escaping.len();
      for i in 0..len {
        let item = items_of_escaping.get(i).unwrap();
        if item.contains('=') {
          let (first, second) = item.split_once('=').unwrap();
          e_names.push(first.trim().to_string());
          e_values.push(second.trim().to_string());
          if i + 1 == len {
            break;
          }
          let remain: Vec<String> =
            items_of_escaping[i + 1..].iter().map(|&item| item.to_string()).collect();
          e_values.extend(remain);
          break;
        } else {
          e_names.push(item.trim().to_string());
        }
      }
      name = e_names.join("=");
      values.push(e_values.join("="));
    } else {
      let (first, second) = line.split_once('=').unwrap();
      name = first.trim().to_string();
      values.push(second.trim().to_string());
    }
  }
  if !name.is_empty() {
    let value = values.join("\n").trim().to_string();
    let allow = if value.is_empty() { !clean_empty } else { true };
    if allow {
      map.insert(name, value);
    }
  }
  map
}

pub fn format(template: &str, args: &HashMap<&str, String>) -> String {
  let mut result = template.to_string();
  for (key, value) in args {
    result = result.replace(&format!("{{{}}}", key), value);
  }
  result
}

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

pub fn languages() -> Vec<String> {
  let translations = TRANSLATIONS.read().unwrap();
  translations.keys().cloned().map(String::from).collect()
}
