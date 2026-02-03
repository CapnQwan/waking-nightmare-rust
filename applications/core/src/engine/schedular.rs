use crate::engine::Phase;
use crate::traits::{AccessPattern, System, SystemResources};
use std::any::Any;
use std::collections::HashMap;

pub struct Schedular {
  phases: HashMap<Phase, Vec<Box<dyn System>>>,
}

impl Schedular {
  pub fn new() -> Self {
    Self {
      phases: HashMap::new(),
    }
  }

  pub fn add_system(&mut self, phase: Phase, system: Box<dyn System>) {
    println!("Adding system {:?} {:?}", phase, system.type_id());
    self.phases.entry(phase).or_default().push(system);
    self.validate();
  }

  pub fn validate(&self) {
    for (phase, systems) in &self.phases {
      let mut has_write = false;

      for system in systems {
        match system.access() {
          AccessPattern::WriteWorld if has_write => {
            panic!("Multiple write systems in {:?}", phase);
          }
          AccessPattern::WriteWorld => has_write = true,
          _ => {}
        }
      }
    }
  }

  pub fn tick(&mut self, ctx: &mut SystemResources, phase: Phase) {
    let Some(systems) = self.phases.get_mut(&phase) else {
      return;
    };

    println!("Systems counts - {:?}", systems.len());

    for system in systems {
      system.run(ctx);
    }
  }
}
