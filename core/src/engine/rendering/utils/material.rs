use std::collections::HashMap;

use crate::{engine::Shader, gl::Gles2};

pub struct Material {
  pub shader: Shader,
  pub uniforms: HashMap<String, i32>,
  pub attributes: HashMap<String, i32>,
}

impl Material {
  pub fn utilize(&self) {
    //self.shader.bind(gl);
    // later: set uniforms, bind VAOs, etc.
  }

  pub fn set_uniform_f32(&self, gl: &Gles2, name: &str, value: f32) {
    if let Some(&loc) = self.uniforms.get(name) {
      unsafe {
        gl.Uniform1f(loc, value);
      }
    }
  }

  pub fn set_uniform_vec2(&self, gl: &Gles2, name: &str, value: [f32; 2]) {
    if let Some(&loc) = self.uniforms.get(name) {
      unsafe {
        gl.Uniform2fv(loc, 1, value.as_ptr());
      }
    }
  }

  pub fn set_uniform_vec3(&self, gl: &Gles2, name: &str, value: [f32; 3]) {
    if let Some(&loc) = self.uniforms.get(name) {
      unsafe {
        gl.Uniform3fv(loc, 1, value.as_ptr());
      }
    }
  }
}
