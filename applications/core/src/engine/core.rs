use crate::engine::Schedular;
use crate::traits::{System, SystemContext, SystemResources};
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum Phase {
  Startup,
  AssetLoad,
  FrameStart,
  Input,
  Simulation,
  PostSimulation,
  RenderPrepare,
  Render,
  Editor,
}

pub struct Core {
  phases: Vec<Phase>,
  schedular: Schedular,
  resources: SystemResources,
  // messageBus: MessageBus,
  // eventBus: EventBus,
}

impl Core {
  pub fn new() -> Core {
    Core {
      phases: vec![
        Phase::Startup,
        Phase::AssetLoad,
        Phase::FrameStart,
        Phase::Input,
        Phase::Simulation,
        Phase::PostSimulation,
        Phase::RenderPrepare,
        Phase::Render,
        Phase::Editor,
      ],
      schedular: Schedular::new(),
      resources: HashMap::new(),
    }
  }

  pub fn tick(&mut self) {
    for phase in &self.phases {
      self.schedular.tick(&mut self.resources, *phase);
    }
  }

  pub fn add_system<T: System + 'static>(&mut self, phase: Phase, system: T) {
    self.schedular.add_system(phase, Box::new(system));
  }

  pub fn add_resource<T: SystemContext + 'static>(&mut self, ctx: T) {
    self.resources.insert(TypeId::of::<T>(), Box::new(ctx));
  }
}
