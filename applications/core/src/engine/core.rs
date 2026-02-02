use std::collections::HashMap;
use crate::plugins::SystemContext;
use crate::schedular::Schedular;

pub enum Phase {
  Startup,
  AssetLoad,
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
  // @todo - this should be switched to something more like HashMap<TypeId, Vec<Box<dyn SystemContext>>>
  resources: HashMap<String, Box<dyn SystemContext>>,
  // messageBus: MessageBus,
  // eventBus: EventBus,
}

impl Core {
  pub fn new() -> Core {
    Core {
      phases: vec![
        Phase::Startup,
        Phase::AssetLoad,
        Phase::Input,
        Phase::Simulation,
        Phase::PostSimulation,
        Phase::RenderPrepare,
        Phase::Render,
        Phase::Editor,
      ],
      schedular: Schedular {},
      resources: HashMap::new(),
    }
  }

  pub fn tick(&mut self) {
    for phase in &self.phases {

    }
  }

  pub fn add_system(&mut self) {

  }

  pub fn add_resource(&mut self) {

  }
}
