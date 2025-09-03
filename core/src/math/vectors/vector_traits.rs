// vectors/vector_traits.rs
pub trait Vector {
  fn normalize(&mut self);
  fn normalized(&self) -> Self;
  fn magnitude(&self) -> f32;
}

pub trait VectorInt {
  fn normalize(&mut self);
  fn normalized(&self) -> Self;
  fn magnitude(&self) -> i32;
}