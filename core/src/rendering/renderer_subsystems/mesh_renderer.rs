use std::rc::Rc;

use crate::{gl::Gles2, rendering::utils::Mesh};

pub struct MeshRenderer {
  gl: Rc<Gles2>,
}

impl MeshRenderer {
  pub fn new(gl: Rc<Gles2>) -> Self {
    Self { gl }
  }

  pub fn draw_mesh(mesh: &Mesh) {}
  pub fn bind_mesh_buffers(mesh: &Mesh) {}
}
