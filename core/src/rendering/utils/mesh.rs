use crate::math::{Vector2, Vector3};

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
  pub fn recalculateNormals(&self) {}

  pub fn bind(&self) {}

  pub fn draw(&self) {}
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
