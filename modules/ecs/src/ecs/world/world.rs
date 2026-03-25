use math::Transform;
use crate::{Component, Components, Entity, Resources};

use std::sync::{Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

/// The central ECS container.
///
/// Responsibilities:
/// - Allocate and recycle `Entity` IDs.
/// - Own all components (via `Components`) and global resources (via `Resources`).
///
/// Concurrency model (lock-based, parallel-system-friendly):
/// - Entity allocation uses atomics + a mutex-protected free list.
/// - Component registry and resource map are behind `RwLock`, so systems can
///   access them concurrently.
/// - Individual component storages/resources are also lock-protected internally,
///   enabling finer-grained parallel reads/writes.
///
/// Notes / future improvements:
/// - ID recycling without generations can create "stale entity" bugs.
/// - For performance, you may later avoid the outer `RwLock<Components>` and
///   rely purely on per-storage locks (or switch to a borrow-checked query model).
pub struct World {
  /// Next never-before-used entity ID.
  next_id: AtomicUsize,

  /// Recycled entity IDs (free list).
  ///
  /// Protected by a mutex because it is mutated by spawn/despawn.
  available_ids: Mutex<Vec<usize>>,

  /// All component storages.
  ///
  /// Outer `RwLock` protects the storage map itself (creating new storages, iterating storages).
  components: RwLock<Components>,

  /// Global singleton-like data.
  resources: RwLock<Resources>,
}

impl World {
  /// Creates a new empty world.
  pub fn new() -> Self {
    Self {
      next_id: AtomicUsize::new(0),
      available_ids: Mutex::new(Vec::new()),
      components: RwLock::new(Components::new()),
      resources: RwLock::new(Resources::new()),
    }
  }

  /// Spawns a new entity and returns its handle.
  ///
  /// Allocation strategy:
  /// - Prefer recycling IDs from `available_ids`.
  /// - Otherwise allocate a fresh ID from `next_id`.
  ///
  /// Caution:
  /// - Without generations, an old `Entity(id)` can become valid again after recycling.
  pub fn spawn_entity(&self) -> Entity {
    if let Ok(mut ids) = self.available_ids.lock() {
      if let Some(id) = ids.pop() {
        return Entity(id);
      }
    }

    let id = self.next_id.fetch_add(1, Ordering::Relaxed);
    Entity(id)
  }

  /// Convenience constructor for a "typical game object".
  ///
  /// Currently:
  /// - Spawns an entity
  /// - Adds a default `Transform`
  ///
  /// As your engine grows, you might replace this with prefabs/bundles.
  pub fn spawn_object(&self) -> Entity {
    let entity = self.spawn_entity();
    if let Ok(mut comps) = self.components.write() {
      comps.add_component::<Transform>(&entity, Transform::default());
    }
    entity
  }

  /// Deletes an entity.
  ///
  /// Behavior:
  /// - Recycles the entity ID
  /// - Removes the entity's components from all storages
  ///
  /// Future:
  /// - If you add parenting/hierarchies, decide whether this cascades to children.
  pub fn delete_entity(&self, entity: Entity) {
    if let Ok(mut ids) = self.available_ids.lock() {
      ids.push(entity.id());
    }
    if let Ok(mut comps) = self.components.write() {
      comps.remove_entity_components(&entity);
    }
  }

  /// Adds/replaces a component on `entity`.
  ///
  /// Locking:
  /// - Takes a write lock on the component registry to reach the storage,
  ///   then the storage takes its own internal lock.
  pub fn add_component<T: Component>(&self, entity: &Entity, component: T) {
    if let Ok(mut comps) = self.components.write() {
      comps.add_component::<T>(entity, component);
    }
  }

  /// Removes a component from `entity` if present.
  pub fn remove_component<T: Component>(&self, entity: &Entity) {
    if let Ok(mut comps) = self.components.write() {
      comps.remove_component::<T>(entity);
    }
  }

  /// Adds a global resource.
  ///
  /// Common usage:
  /// - Insert resources during initialization (time, input state, renderer backend handles).
  pub fn add_resource<T: 'static + Send + Sync>(&self, resource: T) {
    if let Ok(mut res) = self.resources.write() {
      res.add_resource::<T>(resource);
    }
  }

  // Next ergonomic APIs that will help system code:
  // - fn read_components(&self) -> RwLockReadGuard<'_, Components>
  // - fn write_components(&self) -> RwLockWriteGuard<'_, Components>
  // - fn read_resources(&self) -> RwLockReadGuard<'_, Resources>
  // - fn write_resources(&self) -> RwLockWriteGuard<'_, Resources>
}
