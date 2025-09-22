use crate::gl::Gles2;

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
