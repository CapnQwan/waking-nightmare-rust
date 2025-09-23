use crate::gl::Gles2;

// @todo - wrap all gl functionality within a gl_context?
pub unsafe fn create_shader(
  gl: &Gles2,
  shader: gl::types::GLenum,
  source: &[u8],
) -> gl::types::GLuint {
  unsafe {
    let shader = gl.CreateShader(shader);
    gl.ShaderSource(
      shader,
      1,
      [source.as_ptr().cast()].as_ptr(),
      std::ptr::null(),
    );
    gl.CompileShader(shader);
    shader
  }
}

pub fn create_gl_program(
  gl: Gles2,
  vertext_shader_source: &[u8],
  fragment_shader_source: &[u8],
) -> u32 {
  unsafe {
    let vertex_shader = create_shader(&gl, gl::VERTEX_SHADER, vertext_shader_source);
    let fragment_shader = create_shader(&gl, gl::FRAGMENT_SHADER, fragment_shader_source);

    let program = gl.CreateProgram();

    gl.AttachShader(program, vertex_shader);
    gl.AttachShader(program, fragment_shader);

    gl.LinkProgram(program);

    gl.UseProgram(program);

    gl.DeleteShader(vertex_shader);
    gl.DeleteShader(fragment_shader);

    program
  }
}

pub struct Shader {
  program: u32,
}

impl Shader {
  fn bind(&self, gl: Gles2) {
    unsafe {
      gl.UseProgram(self.program);
    }
  }
}
