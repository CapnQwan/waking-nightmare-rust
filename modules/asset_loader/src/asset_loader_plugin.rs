use crate::{SceneLoader, SceneLoaderContext};
use core::engine::Core;
use core::engine::Phase;
use core::traits::Plugin;

pub struct SceneLoaderPlugin {}

impl Plugin for SceneLoaderPlugin {
  fn register(core: &mut Core) {
    core.add_resource(SceneLoaderContext::new());
    core.add_system(Phase::Simulation, SceneLoader::new());
  }
}
