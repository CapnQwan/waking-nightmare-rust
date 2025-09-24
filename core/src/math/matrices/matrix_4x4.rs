// /src/math/matricies/matrix4x4.rs

pub struct Matrix4x4 {
  pub m: [[f32; 4]; 4],
}

impl Default for Matrix4x4 {
  fn default() -> Self {
    Matrix4x4 {
      m: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
      ],
    }
  }
}
