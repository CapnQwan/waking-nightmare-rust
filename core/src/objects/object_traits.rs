// vectors/vector_traits.rs
pub trait ObjectTraits {
  fn instantiate(&self) -> Self;
}

pub trait GameObjectTraits {
  fn addComponent(&self);
}