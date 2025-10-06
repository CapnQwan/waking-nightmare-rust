use std::collections::HashMap;

use crate::engine::Program;

pub struct ProgramRegistry {
  next_id: u32,
  programs: HashMap<u32, Program>,
}

impl ProgramRegistry {
  pub fn new() -> Self {
    Self {
      next_id: 0,
      programs: HashMap::new(),
    }
  }
}
