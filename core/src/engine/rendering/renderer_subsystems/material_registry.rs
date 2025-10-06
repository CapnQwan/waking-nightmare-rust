use std::collections::HashMap;

use crate::{engine::Material, traits::Registry};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MaterialId(u32);

pub struct MaterialRegistry {
  next_id: u32,
  materials: HashMap<MaterialId, Material>,
}

impl Registry<MaterialId, Material> for MaterialRegistry {
  fn new() -> Self {
    Self {
      next_id: 0,
      materials: HashMap::new(),
    }
  }

  fn register(&mut self, mut material: Material) -> MaterialId {
    let id = MaterialId(self.next_id);
    material.set_id(id);
    self.materials.insert(id, material);
    self.next_id += 1;
    id
  }

  fn get(&self, id: &MaterialId) -> Option<&Material> {
    self.materials.get(&id)
  }

  fn get_mut(&mut self, id: &MaterialId) -> Option<&mut Material> {
    self.materials.get_mut(&id)
  }
}
