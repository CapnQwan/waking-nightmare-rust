struct World {
  next_id: u32,
  storages: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
  fn new() -> Self {
    Self {
      next_id: 0,
      storages: HashMap::new(),
    }
  }

  fn spawn(&mut self) -> Entity {
    let e = Entity(self.next_id);
    self.next_id += 1;
    e
  }

  fn storage<T: Component>(&mut self) -> &mut HashMap<Entity, T> {
    self
      .storages
      .entry(TypeId::of::<T>())
      .or_insert_with(|| Box::new(HashMap::<Entity, T>::new()));

    self
      .storages
      .get_mut(&TypeId::of::<T>())
      .unwrap()
      .downcast_mut::<HashMap<Entity, T>>()
      .unwrap()
  }

  fn add_component<T: Component>(&mut self, e: Entity, comp: T) {
    self.storage::<T>().insert(e, comp);
  }

  fn get_component<T: Component>(&self, e: Entity) -> Option<&T> {
    self
      .storages
      .get(&TypeId::of::<T>())
      .and_then(|boxed| boxed.downcast_ref::<HashMap<Entity, T>>())
      .and_then(|map| map.get(&e))
  }

  fn get_component_mut<T: Component>(&mut self, e: Entity) -> Option<&mut T> {
    self
      .storages
      .get_mut(&TypeId::of::<T>())
      .and_then(|boxed| boxed.downcast_mut::<HashMap<Entity, T>>())
      .and_then(|map| map.get_mut(&e))
  }
}
