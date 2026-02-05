use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct Resources {
  resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
  pub fn new() -> Self {
    Self {
      resources: HashMap::new(),
    }
  }

  pub fn add_resource<T: 'static>(&mut self, resource: T) {
    self.resources.insert(TypeId::of::<T>(), Box::new(resource));
  }

  pub fn get_resource<T: 'static>(&self) -> Option<&T> {
    self
      .resources
      .get(&TypeId::of::<T>())
      .and_then(|b| b.downcast_ref())
  }

  pub fn get_mut_resource<T: 'static>(&mut self) -> Option<&mut T> {
    self
      .resources
      .get_mut(&TypeId::of::<T>())
      .and_then(|b| b.downcast_mut())
  }
}
