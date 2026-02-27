use core::engine::Phase;
use core::traits::{AccessPattern, System, SystemResources};

pub struct PreRenderer {}

impl PreRenderer {
  pub fn new() -> Self {
    PreRenderer {}
  }

  pub fn tick(&mut self) {
    // Pull all render data from the shared context
    
  }
}

impl System for PreRenderer {
  fn run(&mut self, _ctx: &mut SystemResources) {
    self.tick();
  }

  fn access(&self) -> AccessPattern {
    AccessPattern::ReadOnly
  }
}
