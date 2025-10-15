use std::{ffi::CStr, sync::Arc};

use glwn::gl::Gl;
use log::info;

use crate::{
  engine::{
    MaterialRegistry, MaterialRenderer, MeshRegistry, MeshRenderer, ProgramRegistry,
    ProgramRenderer, RenderComponent,
  },
  traits::Registry,
};

/** @todo
 * would it be better to switch the registries and maybe even the renderers to
 * be a hash map with a look up or something like that?
 *
 * Maybe this would be better depending on how the engine continues to develop
 * for example if a texture registry or other registries end up being added this
 * idea might be better.
 *
 * Maybe even seperating this logic off so that all Assets are stored in a seperate struct
 * might be better that way other non redering assets like audio can be stored all in one clean
 * little abstraction
 */
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

    let program = self.program_registry.get(material.program()).unwrap();
    unsafe {
      self.gl.UseProgram(program.program());
    }

    if mesh.has_changed() {
      info!("BINDING BUFFERS");
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

  pub fn program_registry(&self) -> &ProgramRegistry {
    &self.program_registry
  }

  pub fn program_registry_mut(&mut self) -> &mut ProgramRegistry {
    &mut self.program_registry
  }

  pub fn program_renderer(&self) -> &ProgramRenderer {
    &self.program_renderer
  }

  pub fn program_renderer_mut(&mut self) -> &mut ProgramRenderer {
    &mut self.program_renderer
  }

  pub fn material_renderer(&self) -> &MaterialRenderer {
    &self.material_renderer
  }

  pub fn material_renderer_mut(&mut self) -> &mut MaterialRenderer {
    &mut self.material_renderer
  }

  pub fn material_registry(&self) -> &MaterialRegistry {
    &self.material_registry
  }

  pub fn material_registry_mut(&mut self) -> &mut MaterialRegistry {
    &mut self.material_registry
  }

  pub fn mesh_renderer(&self) -> &MeshRenderer {
    &self.mesh_renderer
  }

  pub fn mesh_renderer_mut(&mut self) -> &mut MeshRenderer {
    &mut self.mesh_renderer
  }

  pub fn mesh_registry(&self) -> &MeshRegistry {
    &self.mesh_registry
  }

  pub fn mesh_registry_mut(&mut self) -> &mut MeshRegistry {
    &mut self.mesh_registry
  }
}
