use crate::{
  engine::{RenderSystem, Systems, TimeSystem, World},
  gl::Gles2,
};

pub struct Core {
  ecs: World,
  systems: Systems,
}

impl Core {
  pub fn new(gl: Gles2) -> Self {
    let mut ecs = World::new();

    ecs.add_resource(TimeSystem::new());
    ecs.add_resource(RenderSystem::new(gl));

    Core {
      ecs,
      systems: Systems::new(),
    }
  }

  pub fn update(&mut self) {
    self.ecs.update_resources();
    self.systems.update(&mut self.ecs);
    self.draw()
  }

  pub fn draw(&mut self) {
    if let Some(renderer) = self.ecs.get_mut_resource::<RenderSystem>() {
      renderer.clear();
      renderer.draw();
    }
  }
}
