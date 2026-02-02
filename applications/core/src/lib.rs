use log::info;

mod engine;
pub use engine::*;
mod traits;
pub use traits::*;
use crate::core::Core;

pub fn create_engine_instance() -> Core {
  let _ = env_logger::Builder::from_default_env().try_init();
  info!("Starting engine instance...");
  Core::new()
}
