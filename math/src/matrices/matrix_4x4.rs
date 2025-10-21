// /src/math/matricies/matrix4x4.rs
use std::{fmt::Display, ops::{Deref, DerefMut, Mul}};

use crate::{Matrix, Matrix3x3, Quaternion, Transform, Vector3};

/** A row major 4x4 matrix */
#[derive(Clone, Copy)]
pub struct Matrix4x4(pub [[f32; 4]; 4]);

impl Matrix4x4 {
  pub fn identity() -> Matrix4x4 {
    Matrix4x4([
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  pub fn translation_from_values(x: f32, y: f32, z:f32) -> Matrix4x4 {
    Matrix4x4([
      [1.0, 0.0, 0.0, x],
      [0.0, 1.0, 0.0, y],
      [0.0, 0.0, 1.0, z],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  pub fn translation_from_vector(position: Vector3) -> Matrix4x4 {
    Matrix4x4([
      [1.0, 0.0, 0.0, position.x],
      [0.0, 1.0, 0.0, position.y],
      [0.0, 0.0, 1.0, position.z],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  pub fn x_rotation_from_euler(angle: f32) -> Matrix4x4 {
    let cos = angle.cos();
    let sin = angle.sin();
    Matrix4x4([
      [1.0, 0.0, 0.0, 0.0],
      [0.0, cos, -sin, 0.0],
      [0.0, sin, cos, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  pub fn y_rotation_from_euler(angle: f32) -> Matrix4x4 {
    let cos = angle.cos();
    let sin = angle.sin();
    Matrix4x4([
      [cos, 0.0, sin, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [-sin, 0.0, cos, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  pub fn z_rotation_from_euler(angle: f32) -> Matrix4x4 {
    let cos = angle.cos();
    let sin = angle.sin();
    Matrix4x4([
      [cos, -sin, 0.0, 0.0],
      [sin, cos, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  pub fn rotation_from_quaternion(mut quaternion: Quaternion) -> Matrix4x4 {
    quaternion.normalize();
    let (x, y, z, w) = (quaternion.x, quaternion.y, quaternion.z, quaternion.w);
    
    Matrix4x4([
      [1.0 - 2.0 * y * y - 2.0 * z * z, 2.0 * x * y - 2.0 * z * w, 2.0 * x * z + 2.0 * y * w, 0.0],
      [2.0 * x * y + 2.0 * z * w, 1.0 - 2.0 * x * x - 2.0 * z * z, 2.0 * y * z - 2.0 * x * w, 0.0],
      [2.0 * x * z - 2.0 * y * w, 2.0 * y * z + 2.0 * x * w, 1.0 - 2.0 * x * x - 2.0 * y * y, 0.0],
      [0.0, 0.0, 0.0, 1.0]
    ])
  }

  pub fn scale_matrix(scale: Vector3) -> Matrix4x4 {
    Matrix4x4([
      [scale.x, 0.0, 0.0, 0.0],
      [0.0, scale.y, 0.0, 0.0],
      [0.0, 0.0, scale.z, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  pub fn transform_matrix(transform: Transform) -> Matrix4x4 {
    let rotation = Matrix4x4::rotation_from_quaternion(transform.rotation());
    let position = Matrix4x4::translation_from_vector(transform.position());
    let scale = Matrix4x4::scale_matrix(transform.scale());
    position.multiply(&rotation.multiply(&scale))
  }

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

  pub fn inverse(&self) -> Matrix4x4 {
    let m = &self.0;

    let mut inv = [[0.0f32; 4]; 4];

    inv[0][0] =  m[1][1] * m[2][2] * m[3][3]
                - m[1][1] * m[2][3] * m[3][2]
                - m[2][1] * m[1][2] * m[3][3]
                + m[2][1] * m[1][3] * m[3][2]
                + m[3][1] * m[1][2] * m[2][3]
                - m[3][1] * m[1][3] * m[2][2];

    inv[0][1] = -m[0][1] * m[2][2] * m[3][3]
                + m[0][1] * m[2][3] * m[3][2]
                + m[2][1] * m[0][2] * m[3][3]
                - m[2][1] * m[0][3] * m[3][2]
                - m[3][1] * m[0][2] * m[2][3]
                + m[3][1] * m[0][3] * m[2][2];

    inv[0][2] =  m[0][1] * m[1][2] * m[3][3]
                - m[0][1] * m[1][3] * m[3][2]
                - m[1][1] * m[0][2] * m[3][3]
                + m[1][1] * m[0][3] * m[3][2]
                + m[3][1] * m[0][2] * m[1][3]
                - m[3][1] * m[0][3] * m[1][2];

    inv[0][3] = -m[0][1] * m[1][2] * m[2][3]
                + m[0][1] * m[1][3] * m[2][2]
                + m[1][1] * m[0][2] * m[2][3]
                - m[1][1] * m[0][3] * m[2][2]
                - m[2][1] * m[0][2] * m[1][3]
                + m[2][1] * m[0][3] * m[1][2];

    inv[1][0] = -m[1][0] * m[2][2] * m[3][3]
                + m[1][0] * m[2][3] * m[3][2]
                + m[2][0] * m[1][2] * m[3][3]
                - m[2][0] * m[1][3] * m[3][2]
                - m[3][0] * m[1][2] * m[2][3]
                + m[3][0] * m[1][3] * m[2][2];

    inv[1][1] =  m[0][0] * m[2][2] * m[3][3]
                - m[0][0] * m[2][3] * m[3][2]
                - m[2][0] * m[0][2] * m[3][3]
                + m[2][0] * m[0][3] * m[3][2]
                + m[3][0] * m[0][2] * m[2][3]
                - m[3][0] * m[0][3] * m[2][2];

    inv[1][2] = -m[0][0] * m[1][2] * m[3][3]
                + m[0][0] * m[1][3] * m[3][2]
                + m[1][0] * m[0][2] * m[3][3]
                - m[1][0] * m[0][3] * m[3][2]
                - m[3][0] * m[0][2] * m[1][3]
                + m[3][0] * m[0][3] * m[1][2];

    inv[1][3] =  m[0][0] * m[1][2] * m[2][3]
                - m[0][0] * m[1][3] * m[2][2]
                - m[1][0] * m[0][2] * m[2][3]
                + m[1][0] * m[0][3] * m[2][2]
                + m[2][0] * m[0][2] * m[1][3]
                - m[2][0] * m[0][3] * m[1][2];

    inv[2][0] =  m[1][0] * m[2][1] * m[3][3]
                - m[1][0] * m[2][3] * m[3][1]
                - m[2][0] * m[1][1] * m[3][3]
                + m[2][0] * m[1][3] * m[3][1]
                + m[3][0] * m[1][1] * m[2][3]
                - m[3][0] * m[1][3] * m[2][1];

    inv[2][1] = -m[0][0] * m[2][1] * m[3][3]
                + m[0][0] * m[2][3] * m[3][1]
                + m[2][0] * m[0][1] * m[3][3]
                - m[2][0] * m[0][3] * m[3][1]
                - m[3][0] * m[0][1] * m[2][3]
                + m[3][0] * m[0][3] * m[2][1];

    inv[2][2] =  m[0][0] * m[1][1] * m[3][3]
                - m[0][0] * m[1][3] * m[3][1]
                - m[1][0] * m[0][1] * m[3][3]
                + m[1][0] * m[0][3] * m[3][1]
                + m[3][0] * m[0][1] * m[1][3]
                - m[3][0] * m[0][3] * m[1][1];

    inv[2][3] = -m[0][0] * m[1][1] * m[2][3]
                + m[0][0] * m[1][3] * m[2][1]
                + m[1][0] * m[0][1] * m[2][3]
                - m[1][0] * m[0][3] * m[2][1]
                - m[2][0] * m[0][1] * m[1][3]
                + m[2][0] * m[0][3] * m[1][1];

    inv[3][0] = -m[1][0] * m[2][1] * m[3][2]
                + m[1][0] * m[2][2] * m[3][1]
                + m[2][0] * m[1][1] * m[3][2]
                - m[2][0] * m[1][2] * m[3][1]
                - m[3][0] * m[1][1] * m[2][2]
                + m[3][0] * m[1][2] * m[2][1];

    inv[3][1] =  m[0][0] * m[2][1] * m[3][2]
                - m[0][0] * m[2][2] * m[3][1]
                - m[2][0] * m[0][1] * m[3][2]
                + m[2][0] * m[0][2] * m[3][1]
                + m[3][0] * m[0][1] * m[2][2]
                - m[3][0] * m[0][2] * m[2][1];

    inv[3][2] = -m[0][0] * m[1][1] * m[3][2]
                + m[0][0] * m[1][2] * m[3][1]
                + m[1][0] * m[0][1] * m[3][2]
                - m[1][0] * m[0][2] * m[3][1]
                - m[3][0] * m[0][1] * m[1][2]
                + m[3][0] * m[0][2] * m[1][1];

    inv[3][3] =  m[0][0] * m[1][1] * m[2][2]
                - m[0][0] * m[1][2] * m[2][1]
                - m[1][0] * m[0][1] * m[2][2]
                + m[1][0] * m[0][2] * m[2][1]
                + m[2][0] * m[0][1] * m[1][2]
                - m[2][0] * m[0][2] * m[1][1];

    let mut det = m[0][0] * inv[0][0] + m[0][1] * inv[1][0] + m[0][2] * inv[2][0] + m[0][3] * inv[3][0];

    if det.abs() < std::f32::EPSILON {
      return Matrix4x4::identity(); // fallback (non-invertible)
    }

    det = 1.0 / det;

    for i in 0..4 {
      for j in 0..4 {
        inv[i][j] *= det;
      }
    }

    Matrix4x4(inv)
  }

  pub fn transpose(&self) -> Matrix4x4 {
    let mut result = Matrix4x4::default();
    for i in 0..4 {
      for j in 0..4 {
        result[i][j] = self[j][i];
      }
    }
    result
  }

  pub fn to_matrix3x3(&self) -> Matrix3x3 {
    Matrix3x3([
      [self[0][0],self[0][1],self[0][2]], 
      [self[1][0],self[1][1],self[1][2]], 
      [self[2][0],self[2][1],self[2][2]]
      ])
  }

  #[rustfmt::skip]
  pub fn as_column_major(&self) -> [[f32; 4]; 4] {
    [
      [self[0][0], self[1][0], self[2][0], self[3][0]], 
      [self[0][1], self[1][1], self[2][1], self[3][1]], 
      [self[0][2], self[1][2], self[2][2], self[3][2]], 
      [self[0][3], self[1][3], self[2][3], self[3][3]],
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

impl Mul for Matrix4x4 {
  fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
    self.multiply(&rhs)
  }
  
  type Output = Matrix4x4;
}

impl Default for Matrix4x4 {
  fn default() -> Self {
    Matrix4x4::identity()
  }
}

impl Display for Matrix4x4 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for i in 0..4 {
      writeln!(
        f,
        "[{:>8.3}, {:>8.3}, {:>8.3}, {:>8.3}]",
        self[i][0], self[i][1], self[i][2], self[i][3]
      )?;
    }
    Ok(())
  }
}
