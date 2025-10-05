pub struct MeshRegistry {
  next_id: u32,
  meshes: HashMap<u32, Mesh>,
}

impl MeshRegistry {
  pub fn new() -> Self {
    Self {
      next_id: 0,
      meshes: HashMap::new(),
    }
  }
}
