use core::gl::Gl;
use std::rc::Rc;
use winit::window::Window;

pub struct Editor {
  gl_window: Option<Window>,
  gl: Rc<Gl>,
}

impl Editor {
  pub fn new(gl_window, gl) -> Self {
    Self {
      gl_window: None,
      gl: None,
    }
  }
}
