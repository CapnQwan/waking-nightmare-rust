use std::collections::HashMap;

use glwn::gl::Gl;

use crate::engine::{MaterialId, ProgramId};

pub struct Material {
  id: Option<MaterialId>,
  pub program_id: ProgramId,
  pub uniforms: HashMap<String, i32>,
  pub attributes: HashMap<String, i32>,
}

impl Material {
  pub fn new(program_id: ProgramId) -> Self {
    Material {
      id: None,
      program_id,
      uniforms: HashMap::new(),
      attributes: HashMap::new(),
    }
  }

  pub fn utilize(&self) {
    //self.shader.bind(gl);
    // later: set uniforms, bind VAOs, etc.
  }

  pub fn id(&self) -> Option<MaterialId> {
    self.id
  }

  pub fn program(&self) -> &ProgramId {
    &self.program_id
  }

  pub fn set_id(&mut self, id: MaterialId) -> &mut Self {
    self.id = Some(id);
    self
  }

  pub fn set_uniform_f32(&self, gl: &Gl, name: &str, value: f32) {
    if let Some(&loc) = self.uniforms.get(name) {
      unsafe {
        gl.Uniform1f(loc, value);
      }
    }
  }

  pub fn set_uniform_vec2(&self, gl: &Gl, name: &str, value: [f32; 2]) {
    if let Some(&loc) = self.uniforms.get(name) {
      unsafe {
        gl.Uniform2fv(loc, 1, value.as_ptr());
      }
    }
  }

  pub fn set_uniform_vec3(&self, gl: &Gl, name: &str, value: [f32; 3]) {
    if let Some(&loc) = self.uniforms.get(name) {
      unsafe {
        gl.Uniform3fv(loc, 1, value.as_ptr());
      }
    }
  }
}
