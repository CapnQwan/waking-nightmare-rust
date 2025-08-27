// vectors/vector2.rs
use super::vector_traits::VectorInt;

pub struct Vector2Int {
  pub x: i32,
  pub y: i32,
}

impl Vector2Int {
  pub fn new(x: i32, y: i32, z: i32) -> Self {
    Vector2Int { x, y, z }
  }
}

impl VectorInt for Vector2Int {
  fn normalize(&mut self) {
    let mag = self.magnitude();
    if mag != 0.0 {
      self.x /= mag;
      self.y /= mag;
    }
  }

  fn normalized(&self) -> Self {
    let mag = self.magnitude();
    if mag == 0.0 {
      *self
    } else {
      Vector2Int {
        x: self.x / mag,
        y: self.y / mag,
      }
    }
  }

  fn magnitude(&self) -> i32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }
}