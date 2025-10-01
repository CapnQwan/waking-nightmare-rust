use std::alloc::System;

use crate::engine::World;

pub struct Systems {
  systems: Vec<Box<dyn FnMut(&mut World)>>,
}

impl Systems {
  pub fn add_system<F>(&mut self, f: F)
  where
    F: FnMut(&mut World) + 'static,
  {
    self.systems.push(Box::new(f));
  }

  pub fn run(&mut self, world: &mut World) {
    for system in &mut self.systems {
      system(world);
    }
  }
}
