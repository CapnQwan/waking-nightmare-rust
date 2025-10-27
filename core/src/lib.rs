use log::info;
use std::sync::Arc;

use glwn::gl::Gl;

pub use crate::engine::Core;

// @Todo
// Delete mod assets once some sort of asset load / project folder support
// has been implemented
mod assets;
mod engine;
mod traits;

pub fn create_engine_instance(gl: Arc<Gl>) -> Core {
  let _ = env_logger::Builder::from_default_env().try_init();
  info!("Starting engine instance...");
  Core::new(gl)
}
