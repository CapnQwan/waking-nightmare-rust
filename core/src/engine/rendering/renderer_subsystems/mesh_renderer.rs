use std::rc::Rc;

use math::{Vector2, Vector3};

use crate::{engine::Mesh, gl::Gles2};

pub struct MeshRenderer {
  gl: Rc<Gles2>,
}

impl MeshRenderer {
  pub fn new(gl: Rc<Gles2>) -> Self {
    Self { gl }
  }

  pub fn draw_mesh(&self, mesh: &Mesh) {
    unsafe {
      self.gl.BindVertexArray(mesh.vao);
      self.gl.DrawElements(
        gl::TRIANGLES,
        mesh.triangles.len() as i32,
        gl::UNSIGNED_INT,
        std::ptr::null(),
      );
      self.gl.BindVertexArray(0);
    }
  }

  pub fn bind_mesh_buffers(&self, mesh: &mut Mesh) {
    unsafe {
      // @perf_test - does the GenVertexArrays need to run every time the mesh is bound?
      // How much of an impact does it have to render times?

      // Generate and bind VAO
      self.gl.GenVertexArrays(1, &mut mesh.vao);
      self.gl.BindVertexArray(mesh.vao);

      // --- Vertex buffer ---
      self.gl.GenBuffers(1, &mut mesh.vbo);
      self.gl.BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
      self.gl.BufferData(
        gl::ARRAY_BUFFER,
        (mesh.verticies.len() * std::mem::size_of::<Vector3>()) as isize,
        mesh.verticies.as_ptr() as *const _,
        gl::STATIC_DRAW,
      );
      self.gl.EnableVertexAttribArray(0);
      self.gl.VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        std::mem::size_of::<Vector3>() as i32,
        std::ptr::null(),
      );

      // --- UV buffer ---
      if !mesh.uvs.is_empty() {
        self.gl.GenBuffers(1, &mut mesh.uvbo);
        self.gl.BindBuffer(gl::ARRAY_BUFFER, mesh.uvbo);
        self.gl.BufferData(
          gl::ARRAY_BUFFER,
          (mesh.uvs.len() * std::mem::size_of::<Vector2>()) as isize,
          mesh.uvs.as_ptr() as *const _,
          gl::STATIC_DRAW,
        );
        self.gl.EnableVertexAttribArray(1);
        self.gl.VertexAttribPointer(
          1,
          2,
          gl::FLOAT,
          gl::FALSE,
          std::mem::size_of::<Vector2>() as i32,
          std::ptr::null(),
        );
      }

      // --- Normal buffer ---
      if !mesh.normals.is_empty() {
        self.gl.GenBuffers(1, &mut mesh.nbo);
        self.gl.BindBuffer(gl::ARRAY_BUFFER, mesh.nbo);
        self.gl.BufferData(
          gl::ARRAY_BUFFER,
          (mesh.normals.len() * std::mem::size_of::<Vector3>()) as isize,
          mesh.normals.as_ptr() as *const _,
          gl::STATIC_DRAW,
        );
        self.gl.EnableVertexAttribArray(2);
        self.gl.VertexAttribPointer(
          2,
          3,
          gl::FLOAT,
          gl::FALSE,
          std::mem::size_of::<Vector3>() as i32,
          std::ptr::null(),
        );
      }

      // --- Index buffer ---
      self.gl.GenBuffers(1, &mut mesh.ibo);
      self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ibo);
      self.gl.BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (mesh.triangles.len() * std::mem::size_of::<u32>()) as isize,
        mesh.triangles.as_ptr() as *const _,
        gl::STATIC_DRAW,
      );

      // Unbind VAO
      self.gl.BindVertexArray(0);
    }

    mesh.has_changed = false;
  }
}
