use std::rc::Rc;

use crate::gl::Gles2;

pub struct MaterialRenderer {
  gl: Rc<Gles2>,
}

impl MaterialRenderer {
  pub fn new(gl: Rc<Gles2>) -> Self {
    MaterialRenderer { gl }
  }
}
