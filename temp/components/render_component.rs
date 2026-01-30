use crate::engine::{MaterialId, MeshId};

#[derive(Clone, Copy)]
pub struct RenderComponent {
  pub mesh_id: MeshId,
  pub material_id: MaterialId,
}

impl RenderComponent {
  pub fn new(mesh_id: MeshId, material_id: MaterialId) -> Self {
    RenderComponent {
      mesh_id,
      material_id,
    }
  }
}
