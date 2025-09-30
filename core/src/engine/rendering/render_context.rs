use std::{ffi::CStr, rc::Rc};

use crate::{
  assets::{
    FRAGMENT_SHADER_SOURCE, PLANE_TRIANGLES, PLANE_VERTEX_DATA, PLANE_VERTICIES,
    VERTEX_SHADER_SOURCE,
  },
  engine::{Mesh, MeshRenderer, ProgramRenderer, Shader},
  gl::Gles2,
};

pub struct RenderContext {
  gl: Rc<Gles2>,
  mesh_renderer: MeshRenderer,
  program_renderer: ProgramRenderer,
  mesh: Mesh,
  shader: Shader,
}

impl RenderContext {
  pub fn new(gl: Gles2) -> Self {
    let gl = Rc::new(gl);

    let mesh_renderer = MeshRenderer::new(Rc::clone(&gl));
    let program_renderer = ProgramRenderer::new(Rc::clone(&gl));

    let mut mesh = Mesh::new();
    mesh.set_vertices(PLANE_VERTICIES.to_vec());
    mesh.set_triangles(PLANE_TRIANGLES.to_vec());

    let program = program_renderer.create_gl_program(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
    let shader = Shader::new(0, program);

    unsafe {
      let color_attrib = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
      gl.VertexAttribPointer(
        color_attrib as gl::types::GLuint,
        3,
        gl::FLOAT,
        0,
        5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
        (2 * std::mem::size_of::<f32>()) as *const () as *const _,
      );
      gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);
    }

    Self {
      gl,
      mesh_renderer,
      program_renderer,
      mesh,
      shader,
    }
  }

  pub fn clear(&self) {
    unsafe {
      self.gl.ClearColor(1.0, 1.0, 1.0, 1.0);
      self.gl.Clear(gl::COLOR_BUFFER_BIT);
    }
  }

  pub fn draw(&mut self) {
    // self.program_renderer.

    if self.mesh.has_changed() {
      self.mesh_renderer.bind_mesh_buffers(&mut self.mesh);
    }
    self.mesh_renderer.draw_mesh(&self.mesh);
  }

  pub fn resize(&self, width: i32, height: i32) {
    unsafe {
      self.gl.Viewport(0, 0, width, height);
    }
  }

  fn get_gl_string(&self, variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
      let s = self.gl.GetString(variant);
      (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
  }
}
