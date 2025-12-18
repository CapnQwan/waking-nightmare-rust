use egui_extras::{Column, TableBuilder};

pub struct EditorWindow {}

impl Default for EditorWindow {
  fn default() -> Self {
    Self {}
  }
}

impl EditorWindow {
  pub fn render_window(&mut self, ctx: &egui::Context) {
    self.render_file_system(ctx);
    self.render_inspector(ctx);
    self.render_scene(ctx);
    self.render_scene_structure(ctx);
  }

  fn render_scene_structure(&mut self, ctx: &egui::Context) {

    egui::SidePanel::left("left_panel")
      .resizable(true)
      .default_width(250.0)
      .width_range(1.0..=800.0)
      .show(ctx, |ui| {
        ui.vertical_centered(|ui| {
          ui.heading("World Structure");
        });
        ui.vertical(|ui| {});
        egui::ScrollArea::vertical().show(ui, |ui| {
            let text_height = egui::TextStyle::Body
                .resolve(ui.style())
                .size
                .max(ui.spacing().interact_size.y);

            let available_height = ui.available_height();

            let mut table = TableBuilder::new(ui)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::remainder())
                .column(Column::auto())
                .min_scrolled_height(0.0)
                .max_scroll_height(available_height);

            let rows: [f32; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        egui::Sides::new().show(
                            ui,
                            |ui| {
                                ui.strong("ID");
                            },
                            |ui| {
                            },
                        );
                    });
                    header.col(|ui| {
                        ui.strong("Name");
                    });
                    header.col(|ui| {
                        ui.strong("Expand V");
                    });
                })
                .body(|mut body| body.heterogeneous_rows(
                    (rows.into_iter()), | mut row | {
                        row.col(|ui| {
                            ui.label(0.to_string());
                        });
                    }
                ));
        });
      });
  }

  fn render_inspector(&mut self, ctx: &egui::Context) {
    egui::SidePanel::right("right_panel")
      .resizable(true)
      .default_width(250.0)
      .width_range(10.0..=800.0)
      .show(ctx, |ui| {
        ui.vertical_centered(|ui| {
          ui.heading("Inspector");
        });
      });
  }

  fn render_file_system(&mut self, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("bottom_panel")
      .resizable(true)
      .default_height(350.0)
      .height_range(350.0..=800.0)
      .show(ctx, |ui| {
        ui.vertical_centered(|ui| {
          ui.heading("Files");
        });
        egui::ScrollArea::vertical().show(ui, |_ui| {});
      });
  }

  fn render_scene(&mut self, ctx: &egui::Context) {}
}
