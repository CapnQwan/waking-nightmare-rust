use glwn::gl::Gl;

pub unsafe fn create_shader(
  gl: &Gl,
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
    check_shader_compile(gl, shader);
    shader
  }
}

unsafe fn check_shader_compile(gl: &Gl, shader: u32) {
  unsafe {
    let mut status = 0;
    gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    if status == 0 {
      let mut len = 0;
      gl.GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

      let mut buffer = vec![0u8; len as usize];
      gl.GetShaderInfoLog(
        shader,
        len,
        std::ptr::null_mut(),
        buffer.as_mut_ptr() as *mut _,
      );

      let log = String::from_utf8_lossy(&buffer);
      log::error!("Shader compile failed:\n{}", log);
    } else {
      println!("Shader compiled successfully.");
    }
  }
}