// /src/math/matricies/matrix3x3.rs

/** A row major 3x3 matrix */
pub struct Matrix3x3(pub [[f32; 3]; 3]);

impl Matrix3x3 {}

impl Default for Matrix3x3 {
  #[rustfmt::skip]
  fn default() -> Self {
    Matrix3x3([
      [1.0, 0.0, 0.0], 
      [0.0, 1.0, 0.0], 
      [0.0, 0.0, 1.0]
    ])
  }
}
