// vectors/vector3Int.rs
use super::vector_traits::Vector;

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

impl Default for Vector3Int {
  fn default() -> Self {
    Vector3Int { x: 0, y: 0, z: 0 }
  }
}

impl Vector for Vector3Int {
  fn magnitude(&self) -> f32 {
    ((self.x * self.x + self.y * self.y + self.z * self.z) as f32).sqrt()
  }
}
