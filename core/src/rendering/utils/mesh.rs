use crate::{
  gl::Gles2,
  math::{Vector2, Vector3},
};

pub struct Mesh {
  pub triangles: Vec<u32>,
  pub verticies: Vec<Vector3>,
  pub uvs: Vec<Vector2>,
  pub normals: Vec<Vector3>,

  // @todo - should these be pub or get a getter for and unmutable
  // ref be created instead
  pub vao: gl::types::GLuint,
  pub vbo: gl::types::GLuint,
  pub uvbo: gl::types::GLuint,
  pub nbo: gl::types::GLuint,
  pub ibo: gl::types::GLuint,

  pub has_changed: bool,
}

impl Mesh {
  pub fn new(gl: &Gles2) -> Mesh {
    let mut vao: gl::types::GLuint = 0;
    let mut ibo: gl::types::GLuint = 0;
    let mut vbo: gl::types::GLuint = 0;
    let mut uvbo: gl::types::GLuint = 0;
    let mut nbo: gl::types::GLuint = 0;

    unsafe {
      // Generate vertex array object
      gl.GenVertexArrays(1, &mut vao);

      // Generate buffers
      gl.GenBuffers(1, &mut ibo);
      gl.GenBuffers(1, &mut vbo);
      gl.GenBuffers(1, &mut uvbo);
      gl.GenBuffers(1, &mut nbo);
    }

    Mesh {
      triangles: Vec::new(),
      verticies: Vec::new(),
      uvs: Vec::new(),
      normals: Vec::new(),
      vao,
      ibo,
      vbo,
      uvbo,
      nbo,
      has_changed: true,
    }
  }

  pub fn set_vertices(&mut self, verticies: Vec<Vector3>) -> &mut Self {
    self.verticies = verticies;
    self.has_changed = true;
    self
  }

  pub fn set_triangles(&mut self, triangles: Vec<u32>) -> &mut Self {
    self.triangles = triangles;
    self.has_changed = true;
    self
  }

  pub fn set_normals(&mut self, normals: Vec<Vector3>) -> &mut Self {
    self.normals = normals;
    self.has_changed = true;
    self
  }

  pub fn set_uvs(&mut self, uvs: Vec<Vector2>) -> &mut Self {
    self.uvs = uvs;
    self.has_changed = true;
    self
  }
}
