use std::sync::Arc;

use glwn::gl::Gl;

use crate::engine::{Material, ShaderReflection, UniformValue};

pub struct MaterialRenderer {
  gl: Arc<Gl>,
}

impl MaterialRenderer {
  pub fn new(gl: Arc<Gl>) -> Self {
    MaterialRenderer { gl }
  }

  pub fn bind_material(&self, material: &Material, reflection: &ShaderReflection) {
    unsafe {
      self.gl.UseProgram(material.program_id().to_u32());

      for (name, value) in &material.uniforms {
        if let Some(uniform) = reflection.uniforms.get(name) {
          match value {
            UniformValue::Float(v) => {
              self.gl.Uniform1f(uniform.location, *v);
            }
            UniformValue::Vec3(v) => {
              self.gl.Uniform3fv(uniform.location, 1, v.as_ptr());
            }
            UniformValue::Mat3(m) => {
              self
                .gl
                .UniformMatrix3fv(uniform.location, 1, gl::FALSE, m.as_ptr() as *const f32);
            }
            UniformValue::Mat4(m) => {
              self
                .gl
                .UniformMatrix4fv(uniform.location, 1, gl::FALSE, m.as_ptr() as *const f32);
            }
          }
        }
      }
    }
  }
}
