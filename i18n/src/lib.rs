use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::RwLock;

#[cfg(feature = "classic")]
pub use self::vendor::classic::parse_translations;
#[cfg(feature = "fluent")]
pub use self::vendor::fluent::FluentArgs;

mod vendor;

// todo: migrate to egui context
static CURRENT_LANGUAGE: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::default()));
static FALLBACK_LANGUAGE: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::default()));

pub fn set_language(locale: &str) {
  let mut current_locale = CURRENT_LANGUAGE.write().unwrap();
  *current_locale = locale.to_string();
}

pub fn get_language() -> String {
  let current_locale = CURRENT_LANGUAGE.read().unwrap();
  current_locale.clone()
}

pub fn set_fallback(locale: &str) {
  let mut current_locale = FALLBACK_LANGUAGE.write().unwrap();
  *current_locale = locale.to_string();
}

pub fn get_fallback() -> String {
  let current_locale = FALLBACK_LANGUAGE.read().unwrap();
  current_locale.clone()
}

#[allow(unreachable_code)]
pub fn languages() -> Vec<String> {
  #[cfg(feature = "fluent")]
  return vendor::fluent::languages();
  #[cfg(feature = "classic")]
  return vendor::classic::languages();
}

pub fn load_translations_from_map(
  language: impl AsRef<str>,
  translations: HashMap<String, String>,
) -> Result<(), String> {
  let mut translations_contents = vec![];
  for (key, value) in translations.iter() {
    let key = if key.contains("=") { key.replace("=", "\\=") } else { key.to_string() };
    translations_contents.push(format!("{} = {}", key, value));
  }
  load_translations_from_text(language, translations_contents.join("\n"))
}

#[allow(unreachable_code)]
pub fn load_translations_from_text(
  language: impl AsRef<str>,
  content: impl AsRef<str>,
) -> Result<(), String> {
  let language = language.as_ref();
  let content = content.as_ref();

  #[cfg(feature = "fluent")]
  return vendor::fluent::load_translations_from_text(language, content);

  #[cfg(feature = "classic")]
  return vendor::classic::load_translations_from_text(language, content);
}

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
          log::warn!("faild to get path: {:?}", e);
          continue;
        },
      };
      let allow_ext = if let Some(ext) = path_file.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        ext == "egl" || ext == "ftl"
      } else {
        false
      };
      if !allow_ext {
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

    match fs::read_to_string(file) {
      Ok(content) => load_translations_from_text(name, content)?,
      Err(e) => return Err(format!("{:?}", e)),
    };
  }
  Ok(())
}

//#===== transalte

#[cfg(feature = "classic")]
pub fn translate_classic(key: &str, args: &HashMap<&str, String>) -> String {
  let language = get_language();
  let fallback = get_fallback();
  vendor::classic::translate(language, fallback, key, args)
}

#[cfg(feature = "fluent")]
pub fn translate_fluent(key: &str, args: &vendor::fluent::FluentArgs) -> String {
  let language = get_language();
  let fallback = get_fallback();
  vendor::fluent::translate(language, fallback, key, args)
}

#[cfg(not(any(feature = "fluent")))]
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
    let mut args = $crate::FluentArgs::new();
    $(
        args.set(stringify!($name), $val);
    )*
    $crate::translate_fluent($key, &args)
  }};
  ($key:expr) => {{
    $crate::translate_fluent($key, &fluent::FluentArgs::new())
  }};
}
