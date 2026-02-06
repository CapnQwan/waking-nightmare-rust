use crate::Entity;
use sparse_set::sparse_set::SparseSet;
use std::any::Any;

/// Type-erased interface over `ComponentStorage<T>`.
///
/// Why this exists:
/// - `World` needs to keep "a storage per component type" in a single map keyed
///   by `TypeId`.
/// - Rust needs a single concrete type to store in the map, so we erase `T` with `dyn`.
///
/// Threading:
/// - This trait is `Send + Sync` so storages can be put behind locks and accessed
///   from systems running in parallel.
/// - Actual thread-safety is enforced at a higher layer via `RwLock` guards.
pub trait ErasedStorage: Any + Send + Sync {
  /// Expose `Any` for downcasting to the concrete `ComponentStorage<T>`.
  fn as_any(&self) -> &dyn Any;

  /// Expose `Any` for downcasting to the concrete `ComponentStorage<T>`.
  fn as_any_mut(&mut self) -> &mut dyn Any;

  /// Removes the component for `entity` if present.
  fn remove(&mut self, entity: &Entity);

  /// Returns `true` if `entity` currently has a component in this storage.
  fn has(&self, entity: &Entity) -> bool;
}

/// Storage for a single component type `T`, backed by a sparse set.
///
/// Sparse sets are a common ECS choice:
/// - Fast insert/remove by entity ID.
/// - Dense storage for iteration-friendly data locality.
/// - "Hole-free" dense arrays (depending on your `SparseSet` implementation).
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

  // Future ergonomic additions that systems will want quickly:
  // - iter() / iter_mut() over dense data
  // - iter_entities() to also get entity IDs
  // - drain_removed() if you implement change tracking
}

impl<T: 'static + Send + Sync> ErasedStorage for ComponentStorage<T> {
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

impl<T> IntoIterator for ComponentStorage<T> {
  type Item = T;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  /// Consumes the set and yields values in dense order.
  ///
  /// This is mainly useful for tests/debug. Systems usually want non-consuming
  /// iterators so they can iterate every frame without moving the storage.
  fn into_iter(self) -> Self::IntoIter {
    self.components.into_iter()
  }
}