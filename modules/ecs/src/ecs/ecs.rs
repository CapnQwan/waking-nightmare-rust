use core::traits::AccessPattern;
use core::traits::SystemResources;
use core::traits::System;
use crate::{Systems, World};


pub struct ECS {
  systems: Systems,
}

impl ECS {
  pub fn new() -> Self {
    ECS { systems: Systems::new() }
  }
}


impl System for ECS {
  fn run(&mut self, ctx: &mut SystemResources) {
    self.systems.update(&self.world);
  }

  fn access(&self) -> AccessPattern {
    AccessPattern::WriteWorld
  }
}