use crate::Entity;
use sparse_set::sparse_set::SparseSet;
use std::any::Any;

pub trait ErasedStorage: Any {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;

  fn remove(&mut self, entity: &Entity);
  fn has(&self, entity: &Entity) -> bool;
}

pub struct ComponentStorage<T> {
  components: SparseSet<T>,
}

impl<T> ComponentStorage<T> {
  pub fn new() -> Self {
    ComponentStorage {
      components: SparseSet::new(),
    }
  }

  pub fn get(&self, entity: &Entity) -> Option<&T> {
    self.components.get(entity.id())
  }

  pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut T> {
    self.components.get_mut(entity.id())
  }

  pub fn insert(&mut self, entity: &Entity, component: T) {
    self.components.insert(entity.id(), component);
  }

  // pub fn iter(&self)
  // pub fn iter_mut(&mut self)
}

impl<T: 'static> ErasedStorage for ComponentStorage<T> {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }

  fn remove(&mut self, entity: &Entity) {
    self.components.extract(entity.id());
  }

  fn has(&self, entity: &Entity) -> bool {
    self.components.get(entity.id()).is_some()
  }
}
