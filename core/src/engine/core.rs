use crate::{
  engine::{RenderContext, TimeSystem, World},
  gl::Gles2,
};

pub struct Core {
  ecs: World,
  pub time_ctx: TimeSystem,
  pub render_ctx: RenderContext,
}

impl Core {
  pub fn new(gl: Gles2) -> Self {
    Core {
      ecs: World::new(),
      time_ctx: TimeSystem::new(),
      render_ctx: RenderContext::new(gl),
    }
  }

  pub fn update() {
    // handle update logic here
  }
}
