#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

pub fn init() -> eframe::Result {
  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
    ..Default::default()
  };
  eframe::run_native(
    "Waking Nightmare Editor",
    options,
    Box::new(|_| {
      // This gives us image support:
      Ok(Box::<MyApp>::default())
    }),
  )
}

struct MyApp {
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

    egui::SidePanel::left("left_panel")
      .resizable(true)
      .default_width(150.0)
      .width_range(80.0..=200.0)
      .show(ctx, |ui| {
        ui.vertical_centered(|ui| {
          ui.heading("Scene Structure");
        });
        egui::ScrollArea::vertical().show(ui, |_ui| {
            
        });
      });

    egui::SidePanel::right("right_panel")
      .resizable(true)
      .default_width(150.0)
      .width_range(80.0..=200.0)
      .show(ctx, |ui| {
        ui.vertical_centered(|ui| {
          ui.heading("Inspector");
        });
        egui::ScrollArea::vertical().show(ui, |_ui| {
        });
      });

    egui::TopBottomPanel::bottom("bottom_panel")
      .resizable(true)
      .default_height(150.0)
      .height_range(80.0..=200.0)
      .show(ctx, |ui| {
        ui.vertical_centered(|ui| {
          ui.heading("Files");
        });
        egui::ScrollArea::vertical().show(ui, |_ui| {
        });
      });

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Scene");
    });
  }
}
