use log::info;
use std::sync::Arc;

use glwn::gl::Gl;

pub use crate::engine::Core;

// @todo - delete later opting for game (project) based assets to keep core clean
mod assets;
mod engine;
mod traits;

pub fn create_engine_instance(gl: Arc<Gl>) -> Core {
  let _ = env_logger::try_init();
  info!("Starting engine instance...");
  Core::new(gl)
}
