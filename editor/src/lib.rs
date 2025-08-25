#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

pub fn init() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
    ..Default::default()
  };
  eframe::run_native(
    "My egui App",
    options,
    Box::new(|_| {
      // This gives us image support:
      Ok(Box::<MyApp>::default())
    }),
  )
}

struct MyApp {
  name: String,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      name: "CapnQwan".to_owned(),
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("My egui Application");
      ui.horizontal(|ui| {
        let name_label = ui.label("Your name: ");
        ui.text_edit_singleline(&mut self.name)
          .labelled_by(name_label.id);
      });
    });
  }
}
