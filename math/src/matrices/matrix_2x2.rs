// /src/math/matricies/matrix2x2.rs

/** A row major 2x2 matrix */
pub struct Matrix2x2(pub [[f32; 2]; 2]);

impl Matrix2x2 {}

impl Default for Matrix2x2 {
  #[rustfmt::skip]
  fn default() -> Self {
    Matrix2x2([
      [1.0, 0.0], 
      [0.0, 1.0]
    ])
  }
}
