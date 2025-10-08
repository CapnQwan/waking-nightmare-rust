use math::Matrix4x4;

pub struct Camera {
  // @todo iplement a way of outputing different cameras to different outputs (gl_context, render_texture...)
  //output: OutputId?
  field_of_view: u16,
  near: f32,
  far: f32,
  view_matrix: Matrix4x4,
  projection_matrix: Matrix4x4,
}

impl Camera {
  pub fn new(field_of_view: u16, near: f32, far: f32) -> Self {
    Camera {
      field_of_view,
      near,
      far,
      view_matrix,
      projection_matrix,
    }
  }

  pub fn calculate_projection_matrix(&mut self) {}
}

impl Default for Camera {
  fn default() -> Camera {
    Self::new(45, 0.1, 1000.0)
  }
}
