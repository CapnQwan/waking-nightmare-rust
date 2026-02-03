use core::traits::AccessPattern;
use core::traits::SystemResources;
use core::traits::System;
use crate::{Systems, World};


pub struct ECS {
  world: World,
  systems: Systems,
}

impl ECS {
  pub fn new() -> Self {
    let world = World::new();
    let systems = Systems::new();

    ECS { world, systems }
  }
}


impl System for ECS {
  fn run(&mut self, ctx: &mut SystemResources) {
    self.systems.update(&mut self.world);
  }

  fn access(&self) -> AccessPattern {
    AccessPattern::WriteWorld
  }
}