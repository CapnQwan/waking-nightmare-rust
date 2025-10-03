use std::collections::HashMap;

use math::Transform;

use crate::engine::{Component, Components, Entity, Resources};

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
    let e = Entity(self.next_id);
    self.next_id += 1;
    e
  }

  pub fn spawn_object(&mut self) -> Entity {
    let e = Entity(self.next_id);
    self.next_id += 1;
    self.add_component::<Transform>(e, Transform::default());
    e
  }

  pub fn storage<T: Component>(&mut self) -> &mut HashMap<Entity, T> {
    self.components.storage::<T>()
  }

  pub fn add_component<T: Component>(&mut self, e: Entity, comp: T) {
    self.storage::<T>().insert(e, comp);
  }

  pub fn get_component<T: Component>(&self, e: &Entity) -> Option<&T> {
    self.components.get_component::<T>(e)
  }

  pub fn get_component_mut<T: Component>(&mut self, e: &Entity) -> Option<&mut T> {
    self.components.get_component_mut::<T>(e)
  }

  pub fn get_components<T: Component>(&self) -> Option<&HashMap<Entity, T>> {
    self.components.get_components::<T>()
  }

  pub fn get_components_mut<T: Component>(&mut self) -> Option<&mut HashMap<Entity, T>> {
    self.components.get_components_mut::<T>()
  }

  pub fn add_resource<T: 'static>(&mut self, resource: T) {
    self.resources.add_resource::<T>(resource);
  }

  pub fn get_resource<T: 'static>(&self) -> Option<&T> {
    self.resources.get_resource::<T>()
  }

  pub fn get_mut_resource<T: 'static>(&mut self) -> Option<&mut T> {
    self.resources.get_mut_resource::<T>()
  }

  pub fn update_resources(&mut self) {
    self.resources.update_resources();
  }
}
