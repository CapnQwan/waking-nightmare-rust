use math::{Matrix4x4, Transform, Vector3};

// @Todo
// Added support for orthographic cameras
pub struct Camera {
  // @Todo
  // iplement a way of outputing different cameras to different outputs
  // (gl_context, render_texture...)
  //output: OutputId?
  field_of_view: f32,
  near: f32,
  far: f32,
  view_matrix: Matrix4x4,
  is_projection_dirty: bool,
  projection_matrix: Matrix4x4,
  view_projection_matrix: Matrix4x4,
  // @Todo
  // do some more research as to whether the position is needed or
  // if this can just be extracted
  position: Vector3,
}

impl Camera {
  pub fn new(field_of_view: f32, near: f32, far: f32) -> Self {
    let mut camera = Camera {
      field_of_view,
      near,
      far,
      view_matrix: Matrix4x4::default(),
      is_projection_dirty: true,
      projection_matrix: Matrix4x4::default(),
      view_projection_matrix: Matrix4x4::default(),
      position: Vector3::default(),
    };

    camera.update_projection();
    camera
  }

  pub fn position(&self) -> &Vector3 {
    &self.position
  }

  pub fn view_matrix(&self) -> &Matrix4x4 {
    &self.view_matrix
  }

  pub fn projection_matrix(&self) -> &Matrix4x4 {
    &self.projection_matrix
  }

  pub fn view_projection_matrix(&self) -> &Matrix4x4 {
    &self.view_projection_matrix
  }

  pub fn set_view_matrix(&mut self, view: Matrix4x4) {
    self.view_matrix = view;
  }

  pub fn set_field_of_view(&mut self, field_of_view: f32) {
    self.field_of_view = field_of_view;
    self.is_projection_dirty = true;
  }

  pub fn set_near(&mut self, near: f32) {
    self.near = near;
    self.is_projection_dirty = true;
  }

  pub fn set_far(&mut self, far: f32) {
    self.far = far;
    self.is_projection_dirty = true;
  }

  pub fn update_projection(&mut self) {
    if self.is_projection_dirty {
      self.projection_matrix = self.calculate_perspective_projection_matrix();
      self.update_view_projection_matrix();
      self.is_projection_dirty = false;
    }
  }

  pub fn update_view_matrix(&mut self, transform: Transform) {
    self.position = transform.position();
    self.view_matrix = transform.inverse_matrix();
    self.update_view_projection_matrix();
  }

  pub fn update_view_projection_matrix(&mut self) {
    self.view_projection_matrix = self.view_matrix.multiply(&self.projection_matrix);
  }

  pub fn calculate_perspective_projection_matrix(&mut self) -> Matrix4x4 {
    // @Todo
    // implement a way of getting the aspect ratio from the output item
    // let aspect_ratio = view_port._width / view_port._height;
    let aspect_ratio = 1920.0 / 1080.0;

    let fov_rad = self.field_of_view.to_radians();
    let focal_scale = 1.0 / f32::tan(fov_rad / 2.0);
    let range_inv = 1.0 / (self.near - self.far);

    // prettier-ignore
    let mut projection_matrix = Matrix4x4::default();
    projection_matrix[0][0] = focal_scale / aspect_ratio;
    projection_matrix[1][1] = focal_scale;
    projection_matrix[2][2] = (self.far + self.near) * range_inv;
    projection_matrix[2][3] = 2.0 * self.far * self.near * range_inv;
    projection_matrix[3][2] = -1.0;
    projection_matrix[3][3] = 0.0;

    projection_matrix
  }
}

impl Default for Camera {
  fn default() -> Camera {
    Self::new(45.0, 0.1, 1000.0)
  }
}
