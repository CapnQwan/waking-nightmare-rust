use std::collections::HashMap;

use math::Transform;

use crate::engine::{Component, Components, Entity, Resources};

pub struct World {
  next_id: u32,
  components: Components,
  resources: Resources,
}

// @Todo
// Implement a way of destroying entities
//
// @Note
// When implementing a way of destorying entities consider adding a way of
// prioratizing using destroyed entities when creating new entities over just using
// a new entity
impl World {
  pub fn new() -> Self {
    Self {
      next_id: 0,
      components: Components::new(),
      resources: Resources::new(),
    }
  }

  pub fn spawn_entity(&mut self) -> Entity {
    let entity = Entity(self.next_id);
    self.next_id += 1;
    entity
  }

  pub fn spawn_object(&mut self) -> Entity {
    let entity = Entity(self.next_id);
    self.next_id += 1;
    self
      .components
      .add_component::<Transform>(entity, Transform::default());
    entity
  }

  pub fn storage<T: Component>(&mut self) -> &mut HashMap<Entity, T> {
    self.components.storage::<T>()
  }

  pub fn split_borrow(&mut self) -> (&mut Components, &mut Resources) {
    let (components, resources) = (&mut self.components, &mut self.resources);
    (components, resources)
  }

  pub fn add_resource<T: 'static>(&mut self, resource: T) {
    self.resources.add_resource::<T>(resource);
  }

  // @Todo
  // Add threading support
  pub fn update_resources(&mut self) {
    self.resources.update_resources();
  }
}
