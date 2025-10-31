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
        egui::ScrollArea::vertical().show(ui, |_ui| {});
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
          ui.color_edit_button_rgb(&mut [1.0, 1.0, 1.0]);
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
