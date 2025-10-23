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

  pub fn get_component<T: Component>(&self, e: &Entity) -> Option<&T> {
    self
      .components
      .get(&TypeId::of::<T>())
      .and_then(|boxed| boxed.downcast_ref::<HashMap<Entity, T>>())
      .and_then(|map| map.get(&e))
  }

  pub fn get_component_mut<T: Component>(&mut self, e: &Entity) -> Option<&mut T> {
    self
      .components
      .get_mut(&TypeId::of::<T>())
      .and_then(|boxed| boxed.downcast_mut::<HashMap<Entity, T>>())
      .and_then(|map| map.get_mut(&e))
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

  /** @todo - implement a way of destroying a certain struct for an entity */
  pub fn destroy(e: Entity) {
    todo!()
  }

  /** @todo - implement a way of destroying all components for the entity */
  pub fn destroy_all(e: Entity) {
    todo!()
  }

  /**
   * @TODO
   * This could get annoying and cumbersome adding heaps of different functions for getting
   * 2, 3, 4... borrows at once as the engine grows, Need to test and learn if there is a
   * more flexible and managable way of doing this in rust
   */
  pub fn get_two_mut<A: Component, B: Component>(
    &mut self,
  ) -> Option<(&mut HashMap<Entity, A>, &mut HashMap<Entity, B>)> {
    let id_a = TypeId::of::<A>();
    let id_b = TypeId::of::<B>();

    if id_a == id_b {
      // Same type, cannot safely borrow mutably twice
      return None;
    }

    // Get raw pointers to each storage (creating them if needed)
    let a_ptr = self as *mut Self;
    let storage_a = unsafe { (*a_ptr).storage::<A>() as *mut HashMap<Entity, A> };
    let storage_b = unsafe { (*a_ptr).storage::<B>() as *mut HashMap<Entity, B> };

    // SAFETY: Different component types live in different HashMaps
    unsafe { Some((&mut *storage_a, &mut *storage_b)) }
  }
}
