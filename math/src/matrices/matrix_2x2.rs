// /src/math/matricies/matrix2x2.rs

pub struct Matrix2x2 {
  pub m: [[f32; 2]; 2],
}

impl Default for Matrix2x2 {
  fn default() -> Self {
    Matrix2x2 {
      m: [[1.0, 0.0], [0.0, 1.0]],
    }
  }
}
