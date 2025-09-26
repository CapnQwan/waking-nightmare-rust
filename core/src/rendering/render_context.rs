use std::rc::Rc;

use crate::{
  gl::Gles2,
  rendering::{MeshRenderer, ProgramRenderer},
};
//use std::ffi::CString;

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
}
