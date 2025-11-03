use std::collections::HashMap;

use crate::{engine::Mesh, traits::Registry};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MeshId(u32);

pub struct MeshRegistry {
  next_id: u32,
  meshes: HashMap<MeshId, Mesh>,
}

impl Registry<MeshId, Mesh> for MeshRegistry {
  fn new() -> Self {
    Self {
      next_id: 0,
      meshes: HashMap::new(),
    }
  }

  fn register(&mut self, mut mesh: Mesh) -> MeshId {
    let id = MeshId(self.next_id);
    self.next_id += 1;

    mesh.set_id(id);
    self.meshes.insert(id, mesh);
    id
  }

  fn get(&self, id: &MeshId) -> Option<&Mesh> {
    self.meshes.get(id)
  }

  fn get_mut(&mut self, id: &MeshId) -> Option<&mut Mesh> {
    self.meshes.get_mut(id)
  }
}
