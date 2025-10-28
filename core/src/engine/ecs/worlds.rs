use math::Transform;

use crate::engine::{Components, Entity, Resources};

/**
  The World is the heart of the ECS (Entity, Component, System), It's able to create new entities and stores all of the components and recourses for the ECS

  @Todo
  Implement a way of destroying entities

  @Note
  When implementing a way of destorying entities consider adding a way of
  prioratizing using destroyed entities when creating new entities over just using
  a new entity
*/
pub struct World {
  next_id: u32,
  components: Components,
  resources: Resources,
}

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

  // @Todo
  // Delete this once the editor / project parsing has been implemented opting for the spawn
  // entity method to take in a Set of components that initially get set to the entity instead
  pub fn spawn_object(&mut self) -> Entity {
    let entity = self.spawn_entity();
    self
      .components
      .add_component::<Transform>(entity, Transform::default());
    entity
  }

  pub fn split_borrow(&mut self) -> (&mut Components, &mut Resources) {
    let (components, resources) = (&mut self.components, &mut self.resources);
    (components, resources)
  }

  // @Todo
  // Add threading support
  pub fn update_resources(&mut self) {
    self.resources.update_resources();
  }
}
