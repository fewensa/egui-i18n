use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{self, exists, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use syn::visit::Visit;

use crate::types::GenerateOpts;

pub fn generate_languages(opts: GenerateOpts) -> color_eyre::Result<()> {
  let mut generator = Generator { opts };
  generator.generate()
}

struct Generator {
  opts: GenerateOpts,
}

impl Generator {
  fn generate(&mut self) -> color_eyre::Result<()> {
    let source_path = &self.opts.source_path;

    let mut visitor = TranslationVisitor::new();
    let mut extensions = self.opts.extensions.clone();
    if extensions.is_empty() {
      extensions.push("rs".to_string());
    }
    self.visit_dir(Path::new(&source_path), &extensions, &mut visitor);
    self.write_language(visitor.translations)?;
    Ok(())
  }

  fn visit_dir(&self, dir: &Path, extensions: &Vec<String>, visitor: &mut TranslationVisitor) {
    if !dir.is_dir() {
      eprintln!("source path is not a directory");
      return;
    }

    for entry in fs::read_dir(dir).unwrap() {
      let path = entry.unwrap().path();
      if path.is_dir() {
        self.visit_dir(&path, extensions, visitor);
        continue;
      }
      let path_text = path.to_string_lossy().to_string();
      if path_text.contains("/target/") {
        continue;
      }
      let allow = if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_string().to_lowercase();
        extensions.iter().find(|item| item.to_lowercase() == ext).is_some()
      } else {
        false
      };
      if !allow {
        continue;
      }

      self.visit_file(&path, visitor);
    }
  }

  fn visit_file(&self, file: &Path, visitor: &mut TranslationVisitor) {
    let content = fs::read_to_string(file).expect(&format!("Failed to read file: {:?}", file));
    let syntax = syn::parse_file(&content).expect(&format!("Failed to parse file: {:?}", file));
    visitor.visit_file(&syntax);
  }
}

impl Generator {
  fn write_language(&self, translations: Vec<String>) -> color_eyre::Result<()> {
    println!("{:?}", translations);
    let mut languages = self.opts.languages.clone();
    if languages.is_empty() {
      languages.push("en_US".to_string());
    }

    let output_path = match &self.opts.output_path {
      Some(v) => Path::new(v).to_path_buf(),
      None => current_dir()?,
    };
    if !output_path.exists() {
      fs::create_dir_all(&output_path)?;
    }
    let ext = match &self.opts.ext {
      Some(crate::types::LanguageExt::Egl) => "egl",
      Some(crate::types::LanguageExt::Ftl) => "ftl",
      None => "tgl",
    };
    for language in &languages {
      let language_path = output_path.join(&format!("{}.{}", language, ext));
      let mut stored_translations = if language_path.exists() {
        let content = fs::read_to_string(&language_path)?;
        let t = egui_i18n::parse_translations(content, false);
        t.keys().map(|item| item.clone()).collect()
      } else {
        vec![]
      };
      // println!("stored translations: {:?}", stored_translations);
      let mut contents = vec![];
      let is_default = match &self.opts.default_language {
        Some(v) => v == language,
        None => false,
      };
      for t in &translations {
        if stored_translations.contains(t) {
          continue;
        }
        let key = t.replace("=", "\\=");
        if is_default {
          contents.push(format!("{} = {}", key, t));
        } else {
          contents.push(format!("{} =", key));
        }
        stored_translations.push(key);
      }
      if contents.is_empty() {
        continue;
      }
      contents.push("".to_string());

      let mut file = OpenOptions::new().append(true).create(true).open(&language_path)?;
      file.write_all(contents.join("\n").as_bytes())?;
      println!("write transation to: {}", language_path.to_string_lossy());
    }
    Ok(())
  }
}

struct TranslationVisitor {
  translations: Vec<String>,
}

impl TranslationVisitor {
  fn new() -> Self {
    TranslationVisitor { translations: vec![] }
  }

  fn record_translation(&mut self, key: &str) {
    self.translations.push(key.to_string());
  }

  fn is_from_egui_i18n(
    &self,
    segments: &syn::punctuated::Punctuated<syn::PathSegment, syn::token::PathSep>,
  ) -> bool {
    let mut iter = segments.iter();
    let first = iter.next().unwrap();

    if segments.len() < 2 {
      return first.ident == "tr";
    }

    let second = iter.next().unwrap();

    (first.ident == "egui" || first.ident == "egui_i18n") && second.ident == "tr"
  }
}

impl<'ast> Visit<'ast> for TranslationVisitor {
  fn visit_macro(&mut self, mac: &'ast syn::Macro) {
    if self.is_from_egui_i18n(&mac.path.segments) {
      if let Some(first_arg) = extract_first_string_literal(&mac.tokens) {
        self.record_translation(&first_arg);
      }
    }

    syn::visit::visit_macro(self, mac);
  }
}

fn extract_first_string_literal(tokens: &proc_macro2::TokenStream) -> Option<String> {
  let mut iter = tokens.clone().into_iter();

  while let Some(token) = iter.next() {
    if let proc_macro2::TokenTree::Literal(literal) = token {
      if let Ok(syn::Lit::Str(lit_str)) = syn::parse_str::<syn::Lit>(&literal.to_string()) {
        return Some(lit_str.value());
      }
    }
  }
  None
}
