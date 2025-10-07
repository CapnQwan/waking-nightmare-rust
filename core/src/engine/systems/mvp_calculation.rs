use crate::engine::World;

pub fn mvp_calculation(world: &mut World) {
  let (components, _) = world.split_borrow();

  // iterate over all cameras and calculate there view projection matrix
}
