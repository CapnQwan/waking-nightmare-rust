use crate::{
  engine::{Renderer, Systems, Time, World},
  gl::Gles2,
};

pub struct Core {
  world: World,
  systems: Systems,
}

impl Core {
  pub fn new(gl: Gles2) -> Self {
    let mut world = World::new();

    world.add_resource(Time::new());
    world.add_resource(Renderer::new(gl));

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
    if let Some(renderer) = self.world.get_mut_resource::<Renderer>() {
      renderer.clear();
      renderer.draw();
    }
  }
}
