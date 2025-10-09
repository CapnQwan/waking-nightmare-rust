use crate::engine::World;

pub struct Systems {
  systems: Vec<Box<dyn FnMut(&mut World)>>,
}

// @todo - add a way of organizing the execution order of the systems
impl Systems {
  pub fn new() -> Self {
    Systems {
      systems: Vec::new(),
    }
  }

  pub fn add_system<F>(&mut self, f: F) -> &mut Self
  where
    F: FnMut(&mut World) + 'static,
  {
    self.systems.push(Box::new(f));
    self
  }

  pub fn update(&mut self, world: &mut World) {
    for system in &mut self.systems {
      system(world);
    }
  }
}
