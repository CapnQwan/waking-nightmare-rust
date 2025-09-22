use crate::math::FloatVector;

// vectors/vector3.rs
use super::vector_traits::Vector;

#[derive(Clone, Copy)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3 {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Vector3 { x, y, z }
  }
}

impl FloatVector for Vector3 {
  fn normalize(&mut self) {
    let mag = self.magnitude();
    if mag != 0.0 {
      self.x /= mag;
      self.y /= mag;
      self.z /= mag;
    }
  }

  fn normalized(&self) -> Self {
    let mag = self.magnitude();
    if mag == 0.0 {
      *self
    } else {
      Vector3 {
        x: self.x / mag,
        y: self.y / mag,
        z: self.z / mag,
      }
    }
  }
}

impl Vector for Vector3 {
  fn magnitude(&self) -> f32 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }
}
