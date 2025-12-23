use std::any::TypeId;
use std::collections::HashMap;
use crate::traits::Updatable;

pub struct Core {
  systems: HashMap<TypeId, Box<dyn Updatable + 'static>>,
}

// @Todo
// Move all component / entity / system binding to seperate functions.
// Add logic for parsing and saving worlds to some format. Maybe just
// storing all the component data as .rs might be easiest and fastest?
// Need to test perf

impl Core {
  pub fn new() -> Core {
    Core {
      systems: HashMap::new(),
    }
  }
}
