// vectors/vector3Int.rs
use super::vector_traits::VectorInt;

#[derive(Clone, Copy)]
pub struct Vector3Int {
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

impl Vector3Int {
  pub fn new(x: i32, y: i32, z: i32) -> Self {
    Vector3Int { x, y, z }
  }
}

impl VectorInt for Vector3Int {
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
      Vector3Int {
        x: self.x / mag,
        y: self.y / mag,
        z: self.z / mag,
      }
    }
  }

  fn magnitude(&self) -> i32 {
    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
  }
}