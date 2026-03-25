use crate::{ECSContext, ECS};
use core::engine::Core;
use core::engine::Phase;
use core::traits::Plugin;

pub struct ECSPlugin {}

impl Plugin for ECSPlugin {
  fn register(core: &mut Core) {
    core.add_resource(ECSContext::new());
    core.add_system(Phase::Simulation, ECS::new());
  }
}
