use std::collections::HashMap;

use crate::engine::Mesh;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MeshId(u32);

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

  pub fn register(&mut self, mut mesh: Mesh) -> u32 {
    let id = self.next_id;
    self.next_id += 1;

    mesh.set_id(id);
    self.meshes.insert(id, mesh);
    id
  }
}
