use crate::engine::World;

pub fn camera_view_projection_system(world: &mut World) {
  let (components, _) = world.split_borrow();

  // iterate over all cameras and calculate there view projection matrix
}
