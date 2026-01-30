use log::info;
use std::sync::Arc;

use glwn::gl::Gl;

pub use crate::engine::Core;

mod engine;
mod traits;

pub fn create_engine_instance() -> Core {
  let _ = env_logger::Builder::from_default_env().try_init();
  info!("Starting engine instance...");
  Core::new()
}
