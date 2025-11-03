use math::{Quaternion, Transform};

use crate::engine::{RenderComponent, World};

pub fn temp_example_system(world: &mut World) {
  let (components, _) = world.split_borrow();

  let Some((render_components, transforms)) =
    components.get_two_mut::<RenderComponent, Transform>()
  else {
    return;
  };

  for (entity, _) in render_components {
    if let Some(transform) = transforms.get_mut(entity) {
      let x = transform.position().x;
      transform.set_x(if x > 5.0 { -5.0 } else { x + 0.01 });
      let rotation = transform
        .rotation()
        .multiply(Quaternion::identity().rotate_roll(0.001));
      transform.set_rotation(rotation);
      transform.update_world_matrix();
    }
  }
}
