use crate::{ComponentStorage, Entity, ErasedStorage};
use std::{any::TypeId, collections::HashMap};
use std::sync::{Arc, RwLock};

/// Marker trait for component types.
///
/// Requirements:
/// - `'static`: components are stored type-erased and live for the duration of the program.
/// - `Send + Sync`: required if you want to run systems in parallel across threads.
///
/// If you later decide you want non-thread-safe components, you can relax these bounds,
/// but then parallelism must become opt-in / restricted.
pub trait Component: 'static + Send + Sync {}
impl<T: 'static + Send + Sync> Component for T {}

/// The "component registry" for a `World`.
///
/// Internals:
/// - One storage per component type, keyed by `TypeId`.
/// - Each storage is:
///   - type-erased (`Box<dyn ErasedStorage>`) so we can store many types in one map
///   - behind an `RwLock` so systems can read in parallel
///   - wrapped in `Arc` so storages can be cloned/shared easily
///
/// Concurrency model:
/// - Multiple readers can access the same component storage concurrently.
/// - Writers to a specific component type take an exclusive lock for that type.
/// - This is a straightforward "lock-based ECS" approach: easy to get correct,
///   not the absolute fastest, but a good stepping stone to a borrow-checked query API.
pub struct Components {
  storages: HashMap<TypeId, Arc<RwLock<Box<dyn ErasedStorage>>>>,
}

impl Components {
  /// Creates an empty component registry.
  pub fn new() -> Self {
    Components {
      storages: HashMap::new(),
    }
  }

  /// Returns the (type-erased) storage for component type `T`, creating it if missing.
  ///
  /// Why it returns `Arc<RwLock<...>>`:
  /// - Systems can clone the `Arc` and hold onto it.
  /// - Locking happens at the storage level (per component type), which keeps
  ///   contention lower than locking the entire world.
  pub fn storage<T: Component>(&mut self) -> Arc<RwLock<Box<dyn ErasedStorage>>> {
    self
      .storages
      .entry(TypeId::of::<T>())
      .or_insert_with(|| Arc::new(RwLock::new(Box::new(ComponentStorage::<T>::new()))))
      .clone()
  }

  /// Adds/replaces a component on an entity.
  ///
  /// Locking:
  /// - Acquires a write lock for storage `T`.
  pub fn add_component<T: Component>(&mut self, entity: &Entity, component: T) {
    let storage = self.storage::<T>();
    let mut guard = storage.write().unwrap();
    guard
      .as_any_mut()
      .downcast_mut::<ComponentStorage<T>>()
      .expect("ComponentStorage type mismatch")
      .insert(entity, component);
  }

  /// Removes a component from an entity if present.
  ///
  /// Locking:
  /// - Acquires a write lock for storage `T`.
  pub fn remove_component<T: Component>(&mut self, entity: &Entity) {
    let storage = self.storage::<T>();
    let mut guard = storage.write().unwrap();
    guard.remove(entity);
  }

  /// Removes `entity` from *every* component storage.
  ///
  /// This is the core operation needed for `World::delete_entity`.
  ///
  /// Note:
  /// - This iterates all storages, which is O(#component_types). For many games
  ///   this is fine; if it becomes hot, you can track an entity->archetype/signature
  ///   to make deletion faster.
  pub fn remove_entity_components(&mut self, entity: &Entity) {
    for storage in self.storages.values() {
      let mut guard = storage.write().unwrap();
      guard.remove(entity);
    }
  }

  /// Returns `true` if `entity` has component `T`.
  ///
  /// Locking:
  /// - Acquires a read lock for storage `T`.
  pub fn has_component<T: Component>(&mut self, entity: &Entity) -> bool {
    let storage = self.storage::<T>();
    let guard = storage.read().unwrap();
    guard.has(entity)
  }

  /// Convenience read access that clones the component out.
  ///
  /// This is simple, but it forces `T: Clone` and can be expensive for large components.
  /// A more ECS-like API is:
  /// - `with_component<T>(&self, entity, |&T| ...)`
  /// - or a query API returning a lock guard + reference.
  pub fn get_component<T: Component>(&self, e: &Entity) -> Option<T>
  where
    T: Clone,
  {
    self
      .storages
      .get(&TypeId::of::<T>())
      .and_then(|storage| {
        let guard = storage.read().ok()?;
        let typed = guard.as_any().downcast_ref::<ComponentStorage<T>>()?;
        typed.get(e).cloned()
      })
  }

  /// Low-level "give me the write guard" API.
  ///
  /// Caution:
  /// - This returns a guard to the erased storage, not a `&mut T`.
  /// - Callers must downcast inside the guard.
  /// - Prefer higher-level helpers (`with_storage_mut<T>`, queries) as you add them.
  pub fn get_component_mut<T: Component>(
    &mut self,
    _e: &Entity,
  ) -> Option<std::sync::RwLockWriteGuard<'_, Box<dyn ErasedStorage>>> {
    self.storages.get_mut(&TypeId::of::<T>())?.write().ok()
  }
}
