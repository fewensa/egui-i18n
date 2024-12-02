use fluent::FluentArgs;
use intl_memoizer::concurrent::IntlLangMemoizer;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::{collections::HashMap, sync::Arc};

use fluent::{bundle::FluentBundle, FluentResource};

type SharedFluentBundle = Arc<FluentBundle<FluentResource, IntlLangMemoizer>>;

static TRANSLATIONS: Lazy<RwLock<HashMap<String, SharedFluentBundle>>> =
  Lazy::new(|| RwLock::new(HashMap::new()));

pub fn load_translations_from_text(language: impl AsRef<str>, content: impl AsRef<str>) -> Result<(), String> {
  let resource = match FluentResource::try_new(content.as_ref().to_string()) {
    Ok(v) => v,
    Err(e) => {
      return Err(format!("{:?}", e));
    },
  };

  let language_ref = language.as_ref();
  let lang_id = match language_ref.parse() {
    Ok(v) => v,
    Err(e) => {
      return Err(format!("{:?}", e));
    },
  };
  let mut bundle = FluentBundle::new_concurrent(vec![lang_id]);
  if let Err(e) = bundle.add_resource(resource) {
    return Err(format!("{:?}", e));
  }
  let mut translations_map = TRANSLATIONS.write().unwrap();
  translations_map.insert(language_ref.to_string(), Arc::new(bundle));
  Ok(())
}

pub fn translate(
  language: impl AsRef<str>,
  fallback_language: impl AsRef<str>,
  key: &str,
  args: &FluentArgs,
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

fn extract_translate(language: impl AsRef<str>, key: &str, args: &FluentArgs) -> String {
  let translations = TRANSLATIONS.read().unwrap();
  let language_ref = language.as_ref();
  if let Some(bundle) = translations.get(language_ref) {
    if let Some(msg) = bundle.get_message(key) {
      if let Some(pattern) = msg.value() {
        let mut errors = vec![];
        let value = bundle.format_pattern(pattern, Some(args), &mut errors);
        return value.to_string();
      }
    }
  }
  String::default()
}

pub fn languages() -> Vec<String> {
  let translations = TRANSLATIONS.read().unwrap();
  translations.keys().cloned().map(String::from).collect()
}

