use std::{ffi::CStr, sync::Arc};

use glwn::gl::Gl;

use crate::{
  engine::{
    MaterialRegistry, MaterialRenderer, MeshRegistry, MeshRenderer, ProgramRegistry,
    ProgramRenderer, RenderComponent,
  },
  traits::Registry,
};

pub struct Renderer {
  gl: Arc<Gl>,
  material_renderer: MaterialRenderer,
  material_registry: MaterialRegistry,
  mesh_renderer: MeshRenderer,
  mesh_registry: MeshRegistry,
  program_renderer: ProgramRenderer,
  program_registry: ProgramRegistry,
}

impl Renderer {
  pub fn new(gl: Arc<Gl>) -> Self {
    let material_renderer = MaterialRenderer::new(gl.clone());
    let mesh_renderer = MeshRenderer::new(gl.clone());
    let program_renderer = ProgramRenderer::new(gl.clone());

    Self {
      gl,
      material_renderer,
      mesh_renderer,
      program_renderer,
      material_registry: MaterialRegistry::new(),
      mesh_registry: MeshRegistry::new(),
      program_registry: ProgramRegistry::new(),
    }
  }

  pub fn clear(&self) {
    unsafe {
      self.gl.ClearColor(1.0, 1.0, 1.0, 1.0);
      self.gl.Clear(gl::COLOR_BUFFER_BIT);
    }
  }

  pub fn draw(&mut self, render_component: &RenderComponent) {
    let (Some(material), Some(mesh)) = (
      self
        .material_registry
        .get_mut(&render_component.material_id),
      self.mesh_registry.get_mut(&render_component.mesh_id),
    ) else {
      return;
    };

    if mesh.has_changed() {
      self.mesh_renderer.bind_mesh_buffers(mesh);
    }
    self.mesh_renderer.draw_mesh(mesh);
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
