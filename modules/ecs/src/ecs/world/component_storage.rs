use crate::Entity;
use sparse_set::sparse_set::SparseSet;
use std::any::Any;

pub trait ErasedStorage: Any {
  fn remove(&mut self, entity: Entity);
  fn has(&self, entity: Entity) -> bool;
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

  pub fn insert(&mut self, entity: Entity, component: T) {
    self.components.insert(entity.id(), component);
  }

  // pub fn iter(&self)
  // pub fn iter_mut(&mut self)
}

impl<T: 'static> ErasedStorage for ComponentStorage<T> {
  fn remove(&mut self, entity: Entity) {
    self.components.extract(entity.id());
  }

  fn has(&self, entity: Entity) -> bool {
    self.components.get(entity.id()).is_some()
  }
}
