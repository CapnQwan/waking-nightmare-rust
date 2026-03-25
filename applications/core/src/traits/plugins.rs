use crate::engine::Core;
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type SystemResources = HashMap<TypeId, Box<dyn SystemContext>>;

pub enum AccessPattern {
  ReadWorld,
  WriteWorld,
  ReadOnly,
}

pub trait Plugin {
  fn register(core: &mut Core);
}

pub trait System {
  fn run(&mut self, ctx: &mut SystemResources);
  fn access(&self) -> AccessPattern;
}

pub trait SystemContext: Any {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl dyn SystemContext {
  pub fn downcast_ref<T: SystemContext>(&self) -> Option<&T> {
    self.as_any().downcast_ref()
  }

  pub fn downcast_mut<T: SystemContext>(&mut self) -> Option<&mut T> {
    self.as_any_mut().downcast_mut()
  }
}

impl<T: Any> SystemContext for T {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
