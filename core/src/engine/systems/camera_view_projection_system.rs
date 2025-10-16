use math::Transform;

use crate::engine::{Camera, World};

pub fn camera_view_projection_system(world: &mut World) {
  let (components, _) = world.split_borrow();

  let cameras_opt = components.get_components_mut::<Camera>();
  let transforms_opt = components.get_components_mut::<Transform>();

  let (Some(cameras), Some(transforms)) = (cameras_opt, transforms_opt) else {
    return;
  };

  for (entity, camera) in cameras {
    if let Some(transform) = transforms.get_component_mut(entity) {
      camera.update_projection();
      // You could also update the camera's view matrix using `transform` here
    }
  }
}
