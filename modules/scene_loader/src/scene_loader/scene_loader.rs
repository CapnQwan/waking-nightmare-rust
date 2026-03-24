use core::traits::AccessPattern;
use core::traits::System;
use core::traits::SystemResources;

pub struct SceneLoader {}

impl SceneLoader {
  pub fn new() -> Self {
    SceneLoader {}
  }
}

impl System for SceneLoader {
  fn run(&mut self, ctx: &mut SystemResources) {
    // If queued scene in context then load it
  }

  fn access(&self) -> AccessPattern {
    AccessPattern::WriteWorld
  }
}
