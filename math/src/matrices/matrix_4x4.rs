// /src/math/matricies/matrix4x4.rs
use std::ops::{Deref, DerefMut};

/** A row major 4x4 matrix */
pub struct Matrix4x4(pub [[f32; 4]; 4]);

impl Matrix4x4 {
  pub fn multiply(&self, other: &Matrix4x4) -> Matrix4x4 {
    let mut result = Matrix4x4([[0.0; 4]; 4]);
    for i in 0..4 {
      for j in 0..4 {
        for k in 0..4 {
          result[i][j] += self[i][k] * other[k][j];
        }
      }
    }
    result
  }

  #[rustfmt::skip]
  fn to_column_major_arr(&self) -> [f32; 16] {
    [
      self[0][0], self[1][0], self[2][0], self[3][0], 
      self[0][1], self[1][1], self[2][1], self[3][1], 
      self[0][2], self[1][2], self[2][2], self[3][2], 
      self[0][3], self[1][3], self[2][3], self[3][3],
    ]
  }
}

impl Deref for Matrix4x4 {
  type Target = [[f32; 4]; 4];
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Matrix4x4 {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Default for Matrix4x4 {
  fn default() -> Self {
    Matrix4x4([
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }
}
