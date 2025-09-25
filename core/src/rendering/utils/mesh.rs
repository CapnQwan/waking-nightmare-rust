use crate::{
  gl::Gles2,
  math::{Vector2, Vector3},
};

pub struct Mesh {
  pub triangles: Vec<u32>,
  pub verticies: Vec<Vector3>,
  pub uvs: Vec<Vector2>,
  pub normals: Vec<Vector3>,

  vao: gl::types::GLuint,
  vbo: gl::types::GLuint,
  uvbo: gl::types::GLuint,
  nbo: gl::types::GLuint,
  ibo: gl::types::GLuint,
}

impl Mesh {
  pub fn new(gl: &Gles2) -> Mesh {
    let mut vao: gl::types::GLuint = 0;
    let mut vbo: gl::types::GLuint = 0;
    let mut uvbo: gl::types::GLuint = 0;
    let mut nbo: gl::types::GLuint = 0;
    let mut ibo: gl::types::GLuint = 0;

    unsafe {
      // Generate vertex array object
      gl.GenVertexArrays(1, &mut vao);

      // Generate buffers
      gl.GenBuffers(1, &mut vbo);
      gl.GenBuffers(1, &mut uvbo);
      gl.GenBuffers(1, &mut nbo);
      gl.GenBuffers(1, &mut ibo);
    }

    Mesh {
      triangles: Vec::new(),
      verticies: Vec::new(),
      uvs: Vec::new(),
      normals: Vec::new(),
      vao,
      vbo,
      uvbo,
      nbo,
      ibo,
    }
  }

  pub fn recalculate_normals(&mut self) {
    // TODO: implement normal calculation from triangles & vertices
  }

  pub fn bind(&self, gl: &Gles2) {
    unsafe {
      gl.BindVertexArray(self.vao);
    }
  }

  pub fn draw(&self, gl: &Gles2) {
    unsafe {
      gl.BindVertexArray(self.vao);
      gl.DrawElements(
        gl::TRIANGLES,
        self.triangles.len() as i32,
        gl::UNSIGNED_INT,
        std::ptr::null(),
      );
    }
  }
}

// impl Default for Mesh {
//   fn default() -> Self {
//     Mesh {
//       triangles: todo!(),
//       verticies: todo!(),
//       uvs: todo!(),
//       normals: todo!(),
//       vao: todo!(),
//       vbo: todo!(),
//       uvbo: todo!(),
//       nbo: todo!(),
//       ibo: todo!(),
//     }
//   }
// }
