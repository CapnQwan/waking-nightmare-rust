pub struct Mesh {
  pub triangles: [u32],
  pub verticies: [Vector3], // Should this be a Vector3 or a f32[]? depends on how
  pub uvs: [Vector2],       // Refer above
  pub normals: [Vector3],   // Refer above

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
