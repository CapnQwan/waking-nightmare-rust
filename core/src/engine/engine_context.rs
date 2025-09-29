use crate::{
  engine::{RenderContext, TimeContext},
  gl::Gles2,
};

pub struct EngineContext {
  pub time_context: TimeContext,
  pub render_context: RenderContext,
}

impl EngineContext {
  pub fn new(gl: Gles2) -> Self {
    EngineContext {
      time_context: TimeContext::new(),
      render_context: RenderContext::new(gl),
    }
  }
}
