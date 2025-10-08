// /src/math/matricies/matrix3x3.rs
use std::ops::{Deref, DerefMut};

/** A row major 3x3 matrix */
pub struct Matrix3x3(pub [[f32; 3]; 3]);

impl Matrix3x3 {
  pub fn multiply(&self, other: &Matrix3x3) -> Matrix3x3 {
    let mut result = Matrix3x3([[0.0; 3]; 3]);
    for i in 0..3 {
      for j in 0..3 {
        for k in 0..3 {
          result[i][j] += self[i][k] * other[k][j];
        }
      }
    }
    result
  }

  #[rustfmt::skip]
  fn to_column_major_arr(&self) -> [f32; 9] {
    [
      self[0][0], self[1][0], self[2][0], 
      self[0][1], self[1][1], self[2][1], 
      self[0][2], self[1][2], self[2][2], 
    ]
  }
}

impl Deref for Matrix3x3 {
  type Target = [[f32; 3]; 3];
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Matrix3x3 {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}


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
