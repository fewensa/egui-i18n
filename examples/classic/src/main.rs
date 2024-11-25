#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

use egui_i18n::tr;

fn main() -> eframe::Result {
  env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
  init();
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 460.0]),
    ..Default::default()
  };
  eframe::run_native(
    "My egui App",
    options,
    Box::new(|cc| {
      // This gives us image support:
      egui_extras::install_image_loaders(&cc.egui_ctx);

      Ok(Box::<MyApp>::default())
    }),
  )
}

fn init() {
  let en_us = String::from_utf8_lossy(include_bytes!("../../../assets/languages/classic/en_US.egl"));
  let zh_cn = String::from_utf8_lossy(include_bytes!("../../../assets/languages/classic/zh_CN.egl"));
  let ja_jp = String::from_utf8_lossy(include_bytes!("../../../assets/languages/classic/ja_JP.egl"));
  egui_i18n::load_translations_from_text("en_US", en_us).unwrap();
  egui_i18n::load_translations_from_text("zh_CN", zh_cn).unwrap();
  egui_i18n::load_translations_from_text("ja_JP", ja_jp).unwrap();

  // 设置初始语言
  egui_i18n::set_language("en_US");
  egui_i18n::set_fallback("en_US");
}

struct MyApp {
  name: String,
  age: u32,
}

impl Default for MyApp {
  fn default() -> Self {
    Self { name: "Arthur".to_owned(), age: 42 }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("My egui Application");
      ui.horizontal(|ui| {
        let name_label = ui.label("Your name: ");
        ui.text_edit_singleline(&mut self.name).labelled_by(name_label.id);
      });
      ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
      if ui.button("Increment").clicked() {
        self.age += 1;
      }
      if ui.button("en_US").clicked() {
        egui_i18n::set_language("en_US");
      }
      if ui.button("zh_CN").clicked() {
        egui_i18n::set_language("zh_CN");
      }
      if ui.button("ja_JP").clicked() {
        egui_i18n::set_language("ja_JP");
      }
      ui.label(format!("Current language: {}", egui_i18n::get_language()));
      ui.label(format!("Fallback language: {}", egui_i18n::get_fallback()));
      ui.label(format!("Hello '{}', age {}", self.name, self.age));
      ui.label(tr!("Hello=, {name}!", {name: &self.name}));
      ui.label(tr!("hello-name", {name: &self.name}));
      ui.label(egui_i18n::tr!("My name is {name} and {age} years old", {
        name: &self.name, age: self.age
      }));

      ui.image(egui::include_image!("../../../assets/images/ferris.png"));
    });
  }
}
