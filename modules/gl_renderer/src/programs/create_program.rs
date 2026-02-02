use std::sync::Arc;
use glwn::gl::Gl;

pub fn create_gl_program(
  gl: Arc<Gl>,
  vertex_shader: gl::types::GLuint,
  fragment_shader: gl::types::GLuint,
) -> u32 {
  unsafe {
    let program = gl.CreateProgram();

    gl.AttachShader(program, vertex_shader);
    gl.AttachShader(program, fragment_shader);

    gl.LinkProgram(program);

    check_program_link(program);

    gl.UseProgram(program);

    gl.DeleteShader(vertex_shader);
    gl.DeleteShader(fragment_shader);

    program
  }
}

unsafe fn check_program_link(gl: Arc<Gl>, program: u32) {
  unsafe {
    let mut status = 0;
    gl.GetProgramiv(program, gl::LINK_STATUS, &mut status);
    if status == 0 {
      let mut len = 0;
      gl.GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

      let mut buffer = vec![0u8; len as usize];
      gl.GetProgramInfoLog(
        program,
        len,
        std::ptr::null_mut(),
        buffer.as_mut_ptr() as *mut _,
      );

      let log = String::from_utf8_lossy(&buffer);
      log::error!("Program link failed:\n{}", log);
    } else {
      log::info!("Program linked successfully.");
    }
  }
}