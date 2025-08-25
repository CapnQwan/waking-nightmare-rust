// vectors/vector2.rs
use super::vector_traits::Vector;

pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

impl Vector2 {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Vector2 { x, y, z }
  }
}

impl Vector for Vector2 {
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
      Vector2 {
        x: self.x / mag,
        y: self.y / mag,
      }
    }
  }

  fn magnitude(&self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }
}