use std::sync::Arc;

use glwn::gl::Gl;

use crate::engine::{Renderer, Systems, Time, World};

pub struct Core {
  world: World,
  systems: Systems,
}

impl Core {
  pub fn new(gl: Arc<Gl>) -> Self {
    let mut world = World::new();
    let (_, resources) = world.split_borrow();

    resources.add_resource(Time::new());
    resources.add_resource(Renderer::new(gl));

    Core {
      world,
      systems: Systems::new(),
    }
  }

  pub fn update(&mut self) {
    self.world.update_resources();
    self.systems.update(&mut self.world);
    self.draw()
  }

  pub fn draw(&mut self) {
    let (_, resources) = self.world.split_borrow();
    if let Some(renderer) = resources.get_mut_resource::<Renderer>() {
      renderer.clear();
    }
  }
}
