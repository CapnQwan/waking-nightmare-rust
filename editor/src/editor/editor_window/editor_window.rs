use eframe::{egui, NativeOptions};
use egui::{
  Color32, Pos2, Rect,
  containers::{Frame},
  emath, epaint,
  epaint::PathStroke,
  pos2, vec2,
};

pub struct EditorFrame {
  options: NativeOptions,
}

impl Default for EditorFrame {
  fn default() -> Self {
    Self {
      options: eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 720.0]),
        ..Default::default()
      }
    }
  }
}

impl eframe::App for EditorFrame {
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
