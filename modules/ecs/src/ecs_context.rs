use std::sync::{Arc, RwLock};
use crate::World;

pub struct ECSContext {
  world: Arc<RwLock<World>>,
}

impl ECSContext {
  pub fn new() -> Self {
    ECSContext {
      world: Arc::new(RwLock::new(World::new())),
    }
  }
}
