// /src/math/matricies/matrix3x3.rs

pub struct Matrix3x3 {
  pub m: [[f32; 3]; 3],
}

impl Default for Matrix3x3 {
  fn default() -> Self {
    Matrix3x3 {
      m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    }
  }
}
