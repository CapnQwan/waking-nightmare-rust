use crate::core::{Core, Phase};

pub enum AccessPattern {
  ReadWorld,
  WriteWorld,
  ReadOnly,
}

pub trait Plugin {
  fn register(core: &mut Core);
}

trait System {
  fn phase(&self) -> Phase;
  fn run(&mut self, ctx: &mut dyn SystemContext);
  fn access(&self) -> AccessPattern;
}

pub trait SystemContext {

}
