use math::Transform;

use crate::engine::{Camera, RenderCommand, RenderComponent, Renderer, World};

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
  let cameras: Vec<(_, _)> = match components.get_components::<Camera>() {
    Some(iter) => iter.into_iter().collect(),
    None => return,
  };

  for (entity, render_component) in render_components {
    if let Some(transform) = components.get_component::<Transform>(entity) {
      for (_, camera) in &cameras {
        renderer.queue_draw(RenderCommand::new(
          render_component.mesh_id,
          render_component.material_id,
          transform.clone(),
          *camera.view_matrix(),
          *camera.projection_matrix(),
        ));
      }
    }
  }
}
