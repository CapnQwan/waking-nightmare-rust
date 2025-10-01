use std::{
  any::{Any, TypeId},
  collections::HashMap,
};

use crate::engine::Entity;

pub trait Component: 'static {}
impl<T: 'static> Component for T {}

pub struct Components {
  components: HashMap<TypeId, Box<dyn Any>>,
}

impl Components {
  pub fn new() -> Self {
    Components {
      components: HashMap::new(),
    }
  }

  pub fn storage<T: Component>(&mut self) -> &mut HashMap<Entity, T> {
    self
      .components
      .entry(TypeId::of::<T>())
      .or_insert_with(|| Box::new(HashMap::<Entity, T>::new()));

    self
      .components
      .get_mut(&TypeId::of::<T>())
      .unwrap()
      .downcast_mut::<HashMap<Entity, T>>()
      .unwrap()
  }

  pub fn add_component<T: Component>(&mut self, e: Entity, comp: T) {
    self.storage::<T>().insert(e, comp);
  }

  pub fn get_component<T: Component>(&self, e: Entity) -> Option<&T> {
    self
      .components
      .get(&TypeId::of::<T>())
      .and_then(|boxed| boxed.downcast_ref::<HashMap<Entity, T>>())
      .and_then(|map| map.get(&e))
  }

  pub fn get_component_mut<T: Component>(&mut self, e: Entity) -> Option<&mut T> {
    self
      .components
      .get_mut(&TypeId::of::<T>())
      .and_then(|boxed| boxed.downcast_mut::<HashMap<Entity, T>>())
      .and_then(|map| map.get_mut(&e))
  }

  pub fn destory(e: Entity) {}

  pub fn destory_all(e: Entity) {}
}
