use log::info;
use crate::engine::Core;

pub mod engine;
pub mod traits;


pub fn create_engine_instance() -> Core {
  let _ = env_logger::Builder::from_default_env().try_init();
  println!("Starting engine instance...");
  Core::new()
}
