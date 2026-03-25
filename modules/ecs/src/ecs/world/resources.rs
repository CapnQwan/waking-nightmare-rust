use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// "Global" singleton-like data accessible to systems (time, input state, renderer, etc.).
///
/// Model:
/// - One value per type `T`, stored by `TypeId`.
/// - Values are type-erased (`Box<dyn Any + Send + Sync>`) and must be downcast to use.
///
/// Threading:
/// - Each resource is stored behind an `RwLock` and shared via `Arc`, allowing:
///   - many systems to read the same resource concurrently
///   - exclusive write when a system needs to mutate it
///
/// Note:
/// - This is a straightforward approach. Later, you may want ergonomics like
///   `get_or_insert_default`, `expect_resource`, and typed wrapper guards.
pub struct Resources {
  resources: HashMap<TypeId, Arc<RwLock<Box<dyn Any + Send + Sync>>>>,
}

impl Resources {
  /// Creates an empty resource map.
  pub fn new() -> Self {
    Self {
      resources: HashMap::new(),
    }
  }

  /// Inserts (or replaces) a resource of type `T`.
  ///
  /// Common usage: insert resources during engine/plugin initialization.
  pub fn add_resource<T: 'static + Send + Sync>(&mut self, resource: T) {
    self
      .resources
      .insert(TypeId::of::<T>(), Arc::new(RwLock::new(Box::new(resource))));
  }

  /// Returns the lockable handle to a resource of type `T`, if present.
  ///
  /// Typical usage:
  /// - `let res = world.resources().get_resource::<Time>().unwrap();`
  /// - `let time = res.read().unwrap();`
  ///
  /// This returns an `Arc` so callers can clone the handle cheaply.
  pub fn get_resource<T: 'static + Send + Sync>(
    &self,
  ) -> Option<Arc<RwLock<Box<dyn Any + Send + Sync>>>> {
    self.resources.get(&TypeId::of::<T>()).cloned()
  }
}
