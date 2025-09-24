#[derive(Clone, Copy)]
pub struct Quaternion {
  pub w: f32,
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Quaternion {
  pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
    Quaternion { w, x, y, z }
  }
}

impl Default for Quaternion {
  fn default() -> Self {
    Quaternion {
      w: 0.0,
      x: 0.0,
      y: 0.0,
      z: 0.0,
    }
  }
}
