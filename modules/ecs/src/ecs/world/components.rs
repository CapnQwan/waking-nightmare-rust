use crate::{ComponentStorage, Entity, ErasedStorage};
use std::{any::TypeId, collections::HashMap};
use std::any::Any;

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
      .as_any_mut()
      .downcast_mut::<ComponentStorage<T>>()
      .expect("ComponentStorage type mismatch")
  }

  pub fn add_component<T: Component>(&mut self, entity: &Entity, component: T) {
    self.storage::<T>().insert(entity, component);
  }

  pub fn get_component<T: Component>(&self, e: &Entity) -> Option<&T> {
    // self.components.get(&TypeId::of::<T>()).and_then(|boxed| boxed.try_into().and_then(Ok(|result| result))).and_then(|storage|)
    self
      .components
      .get(&TypeId::of::<T>())
      .and_then(|boxed| boxed.as_any().downcast_ref::<ComponentStorage<T>>())
      .and_then(|storage| storage.get(e))
  }

  pub fn get_component_mut<T: Component>(&mut self, e: &Entity) -> Option<&mut T> {
    self
      .components
      .get_mut(&TypeId::of::<T>())
      .and_then(|boxed| boxed.as_any_mut().downcast_mut::<ComponentStorage<T>>())
      .and_then(|storage| storage.get_mut(e))
  }

  pub fn get_components<T: Component>(&self) -> Option<&ComponentStorage<T>> {
    self
      .components
      .get(&TypeId::of::<T>())
      .and_then(|boxed| boxed.as_any().downcast_ref::<ComponentStorage<T>>())
  }

  pub fn get_components_mut<T: Component>(&mut self) -> Option<&mut ComponentStorage<T>> {
    self
      .components
      .get_mut(&TypeId::of::<T>())
      .and_then(|boxed| boxed.as_any_mut().downcast_mut::<ComponentStorage<T>>())
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
