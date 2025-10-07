use math::Transform;

use crate::engine::{MaterialId, MeshId, Renderer, World};

pub struct RenderComponent {
  pub mesh_id: MeshId,
  pub material_id: MaterialId,
}

pub fn mesh_render_system(world: &mut World) {
  let (components, resources) = world.split_borrow();

  let renderer = match resources.get_mut_resource::<Renderer>() {
    Some(r) => r,
    None => return,
  };

  let render_components: Vec<(_, _)> = match components.get_components::<RenderComponent>() {
    Some(iter) => iter.into_iter().collect(),
    None => return,
  };

  for (entity, render_component) in render_components {
    if let Some(transform) = components.get_component::<Transform>(entity) {
      renderer.draw(render_component);
    }
  }
}
