#![allow(dead_code)]
use math::{Vector2, Vector3};
use crate::MeshId;

pub struct Mesh {
  pub id: MeshId,

  pub triangles: Vec<u32>,
  pub vertices: Vec<Vector3>,
  pub uvs: Vec<Vector2>,
  pub normals: Vec<Vector3>,

  pub vao: gl::types::GLuint,
  pub vbo: gl::types::GLuint,
  pub uvbo: gl::types::GLuint,
  pub nbo: gl::types::GLuint,
  pub ibo: gl::types::GLuint,

  pub has_changed: bool,
}

impl Mesh {
  pub fn new(id: MeshId) -> Mesh {
    let vao: gl::types::GLuint = 0;
    let ibo: gl::types::GLuint = 0;
    let vbo: gl::types::GLuint = 0;
    let uvbo: gl::types::GLuint = 0;
    let nbo: gl::types::GLuint = 0;

    Mesh {
      id,
      triangles: Vec::new(),
      vertices: Vec::new(),
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

  pub fn mut_vao(&self) -> &gl::types::GLuint {
    &mut self.vao
  }
  pub fn mut_vbo(&self) -> gl::types::GLuint {
    self.vbo
  }
  pub fn mut_uvbo(&self) -> gl::types::GLuint {
    self.uvbo
  }
  pub fn mut_nbo(&self) -> gl::types::GLuint {
    self.nbo
  }
  pub fn mut_ibo(&self) -> gl::types::GLuint {
    self.ibo
  }

  pub fn has_changed(&self) -> bool {
    self.has_changed
  }

  pub fn id(&self) -> MeshId {
    self.id
  }

  pub fn set_vertices(&mut self, vertices: Vec<Vector3>) -> &mut Self {
    self.vertices = vertices;
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
