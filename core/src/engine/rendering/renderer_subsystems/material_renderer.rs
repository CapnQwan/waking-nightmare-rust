use std::sync::Arc;

use glwn::gl::Gl;

pub struct MaterialRenderer {
  gl: Arc<Gl>,
}

impl MaterialRenderer {
  pub fn new(gl: Arc<Gl>) -> Self {
    MaterialRenderer { gl }
  }
}
