// vectors/vector2.rs
use super::vector_traits::Vector;

#[derive(Clone, Copy)]
pub struct Vector2Int {
  pub x: i32,
  pub y: i32,
}

impl Vector2Int {
  pub fn new(x: i32, y: i32) -> Self {
    Vector2Int { x, y }
  }
}

impl Vector for Vector2Int {
  fn magnitude(&self) -> f32 {
    (self.x * self.x + self.y * self.y).sqrt()
  }
}
