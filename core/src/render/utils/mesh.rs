pub struct Mesh {
  pub triangles: [u32],
  pub verticies: [Vector3], // Should this be a Vector3 or a f32[]? depends on how 
  pub uvs: [Vector2], // Refer above
  pub normals:  [Vector3], // Refer above

  vao: u32, // @todo - proper type BufferObject or something?
  vbo: u32, // @todo - proper type BufferObject or something?
  uvbo: u32, // @todo - proper type BufferObject or something?
  nbo: u32, // @todo - proper type BufferObject or something?
  ibo: u32, // @todo - proper type BufferObject or something?

}

impl Mesh {
  pub fn recalculateNormals(&self) {

  }
  
  pub fn bind(&self) {

  }

  pub fn draw(&self) {
    
  }
}