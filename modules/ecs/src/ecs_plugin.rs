use crate::{ECSContext, ECS};
use core::engine::Core;
use core::engine::Phase;
use core::traits::Plugin;

pub struct TimePlugin {}

impl Plugin for TimePlugin {
  fn register(core: &mut Core) {
    core.add_resource(ECSContext::new());
    core.add_system(Phase::FrameStart, ECS::new());
  }
}
