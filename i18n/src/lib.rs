use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::RwLock;

pub use self::vendor::classic::parse_translations;

#[cfg(feature = "fluent")]
pub use fluent;

#[cfg(feature = "fluent")]
pub use fluent_bundle;

mod vendor;

// ---------------------------------------------------------------------------
// Global configuration
// ---------------------------------------------------------------------------

struct Config {
  language: String,
  fallback: String,
  #[cfg(feature = "fluent")]
  use_isolating: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for Config {
  fn default() -> Self {
    Self {
      language: String::new(),
      fallback: String::new(),
      #[cfg(feature = "fluent")]
      use_isolating: true,
    }
  }
}

// todo: migrate to egui context
static CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| RwLock::new(Config::default()));

// ---------------------------------------------------------------------------
// Language / fallback configuration
// ---------------------------------------------------------------------------

pub fn set_language(locale: &str) {
  CONFIG.write().unwrap().language = locale.to_string();
}

pub fn get_language() -> String {
  CONFIG.read().unwrap().language.clone()
}

pub fn set_fallback(locale: &str) {
  CONFIG.write().unwrap().fallback = locale.to_string();
}

pub fn get_fallback() -> String {
  CONFIG.read().unwrap().fallback.clone()
}

// ---------------------------------------------------------------------------
// Fluent-only: isolating marks configuration
// ---------------------------------------------------------------------------

#[cfg(feature = "fluent")]
pub fn set_use_isolating(value: bool) {
  CONFIG.write().unwrap().use_isolating = value;
}

#[cfg(feature = "fluent")]
pub fn get_use_isolating() -> bool {
  CONFIG.read().unwrap().use_isolating
}

// ---------------------------------------------------------------------------
// Loaded-language enumeration
// ---------------------------------------------------------------------------

#[cfg(feature = "fluent")]
pub fn languages() -> Vec<String> {
  vendor::fluent::languages()
}

#[cfg(not(feature = "fluent"))]
pub fn languages() -> Vec<String> {
  vendor::classic::languages()
}

// ---------------------------------------------------------------------------
// Translation loading — from HashMap
// ---------------------------------------------------------------------------

/// Load translations from a plain key-value map.
///
/// Only available in `classic` mode. In `fluent` mode this function returns
/// an error because a flat `HashMap` cannot represent Fluent syntax; use
/// [`load_translations_from_text`] with raw `.ftl` content instead.
#[cfg(not(feature = "fluent"))]
pub fn load_translations_from_map(
  language: impl AsRef<str>,
  translations: HashMap<String, String>,
) -> Result<(), String> {
  vendor::classic::load_translations_from_map(language, translations);
  Ok(())
}

#[cfg(feature = "fluent")]
pub fn load_translations_from_map(
  _language: impl AsRef<str>,
  _translations: HashMap<String, String>,
) -> Result<(), String> {
  Err(
    "load_translations_from_map is not supported in fluent mode; \
     use load_translations_from_text with raw .ftl content instead"
      .to_string(),
  )
}

// ---------------------------------------------------------------------------
// Translation loading — from text
// ---------------------------------------------------------------------------

#[cfg(feature = "fluent")]
pub fn load_translations_from_text(
  language: impl AsRef<str>,
  content: impl AsRef<str>,
) -> Result<(), String> {
  vendor::fluent::load_translations_from_text(
    language.as_ref(),
    content.as_ref(),
    get_use_isolating(),
  )
}

#[cfg(not(feature = "fluent"))]
pub fn load_translations_from_text(
  language: impl AsRef<str>,
  content: impl AsRef<str>,
) -> Result<(), String> {
  vendor::classic::load_translations_from_text(language, content)
}

// ---------------------------------------------------------------------------
// Translation loading — from filesystem path
// ---------------------------------------------------------------------------

/// Load all `.egl` / `.ftl` translation files from a directory (or a single
/// file). Each file's stem is used as the language identifier.
pub fn load_translations_from_path(path: impl AsRef<str>) -> Result<(), String> {
  let path_ref = Path::new(path.as_ref());
  let mut files = vec![];

  if path_ref.is_file() {
    files.push(path_ref.to_path_buf());
  } else {
    let read_dir = match fs::read_dir(path_ref) {
      Ok(v) => v,
      Err(e) => return Err(format!("{:?}", e)),
    };
    for entry in read_dir {
      let path_file = match entry {
        Ok(dir_entry) => dir_entry.path(),
        Err(e) => {
          log::warn!("failed to read directory entry: {:?}", e);
          continue;
        },
      };
      let allowed = path_file
        .extension()
        .map(|ext| {
          let ext = ext.to_string_lossy().to_lowercase();
          ext == "egl" || ext == "ftl"
        })
        .unwrap_or(false);
      if !allowed {
        continue;
      }
      files.push(path_file);
    }
  }

  for file in files {
    let name = match file.file_stem() {
      Some(v) => v.to_string_lossy().to_string(),
      None => continue,
    };
    match fs::read_to_string(&file) {
      Ok(content) => load_translations_from_text(name, content)?,
      Err(e) => return Err(format!("{:?}", e)),
    }
  }
  Ok(())
}

// ---------------------------------------------------------------------------
// Translation execution
// ---------------------------------------------------------------------------

#[cfg(not(feature = "fluent"))]
pub fn translate_classic(key: &str, args: &HashMap<&str, String>) -> String {
  let language = get_language();
  let fallback = get_fallback();
  vendor::classic::translate(language, fallback, key, args)
}

#[cfg(feature = "fluent")]
pub fn translate_fluent(key: &str, args: &crate::fluent::FluentArgs) -> String {
  let language = get_language();
  let fallback = get_fallback();
  vendor::fluent::translate(language, fallback, key, args)
}

// ---------------------------------------------------------------------------
// tr! macro
// ---------------------------------------------------------------------------

#[cfg(not(feature = "fluent"))]
#[macro_export]
macro_rules! tr {
  ($key:expr, {$($name:ident: $val:expr),*}) => {{
    let mut args = std::collections::HashMap::new();
    $(
      args.insert(stringify!($name), $val.to_string());
    )*
    $crate::translate_classic($key, &args)
  }};
  ($key:expr) => {{
    $crate::translate_classic($key, &std::collections::HashMap::new())
  }};
}

#[cfg(feature = "fluent")]
#[macro_export]
macro_rules! tr {
  ($key:expr, {$($name:ident: $val:expr),*}) => {{
    let mut args = $crate::fluent::FluentArgs::new();
    $(
      args.set(stringify!($name), $val);
    )*
    $crate::translate_fluent($key, &args)
  }};
  ($key:expr) => {{
    $crate::translate_fluent($key, &$crate::fluent::FluentArgs::new())
  }};
}
