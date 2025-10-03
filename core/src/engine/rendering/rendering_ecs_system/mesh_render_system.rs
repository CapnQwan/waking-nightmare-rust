use math::Transform;

use crate::engine::{Renderer, World};

struct MeshRenderer {
  mesh_id: u32,
  material_id: u32,
}

pub fn mesh_render_system(world: &mut World) {
  let Some(renderer) = world.get_resource::<Renderer>() else {
    return;
  };

  for (entity, mesh_renderer) in world.get_components::<MeshRenderer>().unwrap() {
    if let Some(transform) = world.get_component::<Transform>(entity) {
      //renderer.draw_mesh(mesh_renderer.mesh_id, &transform);
    }
  }
}
