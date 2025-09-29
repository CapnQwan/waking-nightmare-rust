use std::{ffi::CStr, rc::Rc};

use crate::{
  engine::{MeshRenderer, ProgramRenderer},
  gl::Gles2,
};

pub struct RenderContext {
  gl: Rc<Gles2>,
  mesh_renderer: MeshRenderer,
  program_renderer: ProgramRenderer,
}

impl RenderContext {
  pub fn new(gl: Gles2) -> Self {
    let gl = Rc::new(gl);

    let mesh_renderer = MeshRenderer::new(Rc::clone(&gl));
    let program_renderer = ProgramRenderer::new(Rc::clone(&gl));

    Self {
      gl,
      mesh_renderer,
      program_renderer,
    }
  }

  pub fn clear(&self) {
    unsafe {
      self.gl.ClearColor(1.0, 1.0, 1.0, 1.0);
      self.gl.Clear(gl::COLOR_BUFFER_BIT);
    }
  }

  pub fn draw(&self) {
    // Switch to
    let _x: f32 = 5.0;
  }

  pub fn resize(&self, width: i32, height: i32) {
    unsafe {
      self.gl.Viewport(0, 0, width, height);
    }
  }

  fn get_gl_string(&self, variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
      let s = self.gl.GetString(variant);
      (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
  }
}
