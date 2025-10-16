// /src/math/matricies/matrix4x4.rs
use std::ops::{Deref, DerefMut};

use crate::{Quaternion, Transform, Vector3};

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

  #[rustfmt::skip]
  fn as_column_major(&self) -> [f32; 16] {
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
    Matrix4x4::identity()
  }
}
