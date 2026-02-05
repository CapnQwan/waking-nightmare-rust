use crate::{ComponentStorage, Entity, ErasedStorage};
use std::{any::TypeId, collections::HashMap};

pub trait Component: 'static {}
impl<T: 'static> Component for T {}

pub struct Components {
  components: HashMap<TypeId, Box<dyn ErasedStorage>>,
}

impl Components {
  pub fn new() -> Self {
    Components {
      components: HashMap::new(),
    }
  }

  pub fn storage<T: Component>(&mut self) -> &mut ComponentStorage<T> {
    self
      .components
      .entry(TypeId::of::<T>())
      .or_insert_with(|| Box::new(ComponentStorage::<T>::new()));

    self
      .components
      .get_mut(&TypeId::of::<T>())
      .unwrap()
      .as_mut()
      .downcast_mut::<ComponentStorage<T>>()
      .expect("ComponentStorage type mismatch")

    // self
    //   .components
    //   .get_mut(&TypeId::of::<T>())
    //   .unwrap()
    //   .as_mut()
  }

  pub fn add_component<T: Component>(&mut self, e: Entity, comp: T) {
    self.storage::<T>().insert(e, comp);
  }

  pub fn get_component<T: Component>(&self, e: &Entity) -> Option<&T> {
    // self.components.get(&TypeId::of::<T>()).and_then(|boxed| boxed.try_into().and_then(Ok(|result| result))).and_then(|storage|)
    self
      .components
      .get(&TypeId::of::<T>())
      .and_then(|boxed| boxed.downcast_ref::<ComponentStorage<T>>())
      .and_then(|storage| storage.components.get(e.id()))
  }

  pub fn get_component_mut<T: Component>(&mut self, e: &Entity) -> Option<&mut T> {
    self
      .components
      .get_mut(&TypeId::of::<T>())
      .and_then(|boxed| boxed.downcast_mut::<ComponentStorage<T>>())
      .and_then(|storage| storage.components.get_mut(e.id()))
  }

  pub fn get_components<T: Component>(&self) -> Option<&HashMap<Entity, T>> {
    self
      .components
      .get(&TypeId::of::<T>())
      .and_then(|boxed| boxed.downcast_ref::<HashMap<Entity, T>>())
  }

  pub fn get_components_mut<T: Component>(&mut self) -> Option<&mut HashMap<Entity, T>> {
    self
      .components
      .get_mut(&TypeId::of::<T>())
      .and_then(|boxed| boxed.downcast_mut::<HashMap<Entity, T>>())
  }

  // @Todo
  // implement a way of destroying a certain struct for an entity
  pub fn destroy(e: Entity) {
    todo!()
  }

  // @Todo
  // implement a way of destroying all components for the entity */
  pub fn destroy_all(e: Entity) {
    todo!()
  }
}
