use crate::{gl::Gles2, render::utils::shader::create_shader};

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
