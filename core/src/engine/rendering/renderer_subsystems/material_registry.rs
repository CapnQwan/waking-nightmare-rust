pub struct MaterialRegistry {
  next_id: u32,
  materials: HashMap<u32, Material>,
}

impl MaterialRegistry {
  pub fn new() -> Self {
    Self {
      next_id: 0,
      materials: HashMap::new(),
    }
  }

  pub fn register_material(&mut self, material: Material) -> MaterialId {
    let id = self.next_id;
    self.materials.insert(id, material);
    self.next_id += 1;
    MaterialId(id)
  }

  pub fn get_material(&self, id: MaterialId) -> Option<&Material> {
    self.materials.get(&id.0)
  }
}
