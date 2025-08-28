#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::{
    Color32, Context, Pos2, Rect, Ui,
    containers::{Frame, Window},
    emath, epaint,
    epaint::PathStroke,
    lerp, pos2, remap, vec2,
};

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
      Frame::canvas(ui.style()).show(ui, |ui| {
            ui.ctx().request_repaint();
            let time = ui.input(|i| i.time);

            let desired_size = ui.available_width() * vec2(1.0, 0.35);
            let (_id, rect) = ui.allocate_space(desired_size);

            let to_screen =
                emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);

            let mut shapes = vec![];

            for &mode in &[2, 3, 5] {
                let mode = mode as f64;
                let n = 120;
                let speed = 1.5;

                let points: Vec<Pos2> = (0..=n)
                    .map(|i| {
                        let t = i as f64 / (n as f64);
                        let amp = (time * speed * mode).sin() / mode;
                        let y = amp * (t * std::f64::consts::TAU / 2.0 * mode).sin();
                        to_screen * pos2(t as f32, y as f32)
                    })
                    .collect();

                let thickness = 10.0 / mode as f32;
                shapes.push(epaint::Shape::line(
                    points,
                    PathStroke::new(thickness, Color32::from_rgb(0x20, 0x21, 0x22)),
                ));
            }

            ui.painter().extend(shapes);
        });
    });
  }
}
