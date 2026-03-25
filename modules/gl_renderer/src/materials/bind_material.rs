use glwn::gl::Gl;
use rendering::{Material, UniformValue};
use crate::ShaderReflection;

pub fn bind_material(gl: &Gl, material: &Material, reflection: &ShaderReflection) {
  unsafe {
    for (name, value) in &material.uniforms {
      if let Some(uniform) = reflection.uniforms.get(name) {
        match value {
          UniformValue::Float(v) => {
            gl.Uniform1f(uniform.location, *v);
          }
          UniformValue::Vec3(v) => {
            gl.Uniform3fv(uniform.location, 1, v.as_ptr());
          }
          UniformValue::Mat3(m) => {
            gl.UniformMatrix3fv(uniform.location, 1, gl::FALSE, m.as_ptr() as *const f32);
          }
          UniformValue::Mat4(m) => {
            gl.UniformMatrix4fv(uniform.location, 1, gl::FALSE, m.as_ptr() as *const f32);
          }
        }
      }
    }
  }
}
