// vectors/vector_traits.rs
pub trait Vector {
  fn magnitude(&self) -> f32;
}

pub trait FloatVector {
  fn normalize(&mut self);
  fn normalized(&self) -> Self;
}
