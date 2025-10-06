use std::{ffi::CStr, rc::Rc};

use crate::{
  engine::{
    MaterialId, MaterialRegistry, MaterialRenderer, MeshId, MeshRegistry, MeshRenderer,
    ProgramRegistry, ProgramRenderer, RenderComponent,
  },
  gl::Gles2,
  traits::Registry,
};

pub struct Renderer {
  gl: Rc<Gles2>,
  material_renderer: MaterialRenderer,
  material_registry: MaterialRegistry,
  mesh_renderer: MeshRenderer,
  mesh_registry: MeshRegistry,
  program_renderer: ProgramRenderer,
  program_registry: ProgramRegistry,
}

impl Renderer {
  pub fn new(gl: Gles2) -> Self {
    let gl = Rc::new(gl);

    let material_renderer = MaterialRenderer::new(Rc::clone(&gl));
    let mesh_renderer = MeshRenderer::new(Rc::clone(&gl));
    let program_renderer = ProgramRenderer::new(Rc::clone(&gl));

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
