use std::sync::Arc;

use glwn::gl::Gl;

pub struct ProgramRenderer {
  gl: Arc<Gl>,
}

impl ProgramRenderer {
  pub fn new(gl: Arc<Gl>) -> Self {
    Self { gl }
  }

  pub fn bind_program() {}

  pub fn bind_attributes() {}

  pub fn bind_uniforms() {}

  pub fn bind_textures() {}

  pub unsafe fn create_shader(
    &self,
    shader: gl::types::GLenum,
    source: &[u8],
  ) -> gl::types::GLuint {
    unsafe {
      let shader = self.gl.CreateShader(shader);
      self.gl.ShaderSource(
        shader,
        1,
        [source.as_ptr().cast()].as_ptr(),
        std::ptr::null(),
      );
      self.gl.CompileShader(shader);
      shader
    }
  }

  pub fn create_gl_program(
    &self,
    vertext_shader_source: &[u8],
    fragment_shader_source: &[u8],
  ) -> u32 {
    unsafe {
      let vertex_shader = self.create_shader(gl::VERTEX_SHADER, vertext_shader_source);
      let fragment_shader = self.create_shader(gl::FRAGMENT_SHADER, fragment_shader_source);

      let program = self.gl.CreateProgram();

      self.gl.AttachShader(program, vertex_shader);
      self.gl.AttachShader(program, fragment_shader);

      self.gl.LinkProgram(program);

      self.gl.UseProgram(program);

      self.gl.DeleteShader(vertex_shader);
      self.gl.DeleteShader(fragment_shader);

      program
    }
  }
}
