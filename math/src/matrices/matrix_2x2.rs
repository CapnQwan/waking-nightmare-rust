// /src/math/matricies/matrix2x2.rs
use std::ops::{Deref, DerefMut};

/** A row major 2x2 matrix */
#[derive(Clone, Copy)]
pub struct Matrix2x2(pub [[f32; 2]; 2]);

impl Matrix2x2 {
    pub fn multiply(&self, other: &Matrix2x2) -> Matrix2x2 {
    let mut result = Matrix2x2([[0.0; 2]; 2]);
    for i in 0..2 {
      for j in 0..2 {
        for k in 0..2 {
          result[i][j] += self[i][k] * other[k][j];
        }
      }
    }
    result
  }

  #[rustfmt::skip]
  fn to_column_major_arr(&self) -> [f32; 4] {
    [
      self[0][0], self[1][0],
      self[0][1], self[1][1],
    ]
  }
}

impl Deref for Matrix2x2 {
  type Target = [[f32; 2]; 2];
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Matrix2x2 {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}


impl Default for Matrix2x2 {
  #[rustfmt::skip]
  fn default() -> Self {
    Matrix2x2([
      [1.0, 0.0], 
      [0.0, 1.0]
    ])
  }
}
