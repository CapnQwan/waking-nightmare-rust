use crate::{SceneManager, SceneManagerContext};
use core::engine::Core;
use core::engine::Phase;
use core::traits::Plugin;

pub struct SceneManagerPlugin {}

impl Plugin for SceneManagerPlugin {
  fn register(core: &mut Core) {
    core.add_resource(SceneManagerContext::new());
    core.add_system(Phase::Simulation, SceneManager::new());
  }
}
