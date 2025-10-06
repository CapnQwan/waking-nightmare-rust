use math::{Vector2, Vector3};

use crate::engine::MeshId;

pub struct Mesh {
  id: Option<MeshId>,

  pub(crate) triangles: Vec<u32>,
  pub(crate) verticies: Vec<Vector3>,
  pub(crate) uvs: Vec<Vector2>,
  pub(crate) normals: Vec<Vector3>,

  pub(crate) vao: gl::types::GLuint,
  pub(crate) vbo: gl::types::GLuint,
  pub(crate) uvbo: gl::types::GLuint,
  pub(crate) nbo: gl::types::GLuint,
  pub(crate) ibo: gl::types::GLuint,

  pub(crate) has_changed: bool,
}

impl Mesh {
  pub fn new() -> Mesh {
    let vao: gl::types::GLuint = 0;
    let ibo: gl::types::GLuint = 0;
    let vbo: gl::types::GLuint = 0;
    let uvbo: gl::types::GLuint = 0;
    let nbo: gl::types::GLuint = 0;

    Mesh {
      id: None,
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

  pub fn vao(&self) -> gl::types::GLuint {
    self.vao
  }
  pub fn vbo(&self) -> gl::types::GLuint {
    self.vbo
  }
  pub fn uvbo(&self) -> gl::types::GLuint {
    self.uvbo
  }
  pub fn nbo(&self) -> gl::types::GLuint {
    self.nbo
  }
  pub fn ibo(&self) -> gl::types::GLuint {
    self.ibo
  }

  pub fn has_changed(&self) -> bool {
    self.has_changed
  }

  pub fn id(&self) -> Option<MeshId> {
    self.id
  }

  pub fn set_id(&mut self, id: MeshId) -> &mut Self {
    self.id = Some(id);
    self
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
