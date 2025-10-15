use std::collections::HashMap;

use crate::{engine::Program, traits::Registry};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProgramId(u32);

pub struct ProgramRegistry {
  next_id: u32,
  programs: HashMap<ProgramId, Program>,
}

impl Registry<ProgramId, Program> for ProgramRegistry {
  fn new() -> Self {
    Self {
      next_id: 0,
      programs: HashMap::new(),
    }
  }

  fn register(&mut self, program: Program) -> ProgramId {
    let id = ProgramId(self.next_id);
    self.next_id += 1;
    self.programs.insert(id, program);
    id
  }

  fn get(&self, id: &ProgramId) -> Option<&Program> {
    self.programs.get(id)
  }

  fn get_mut(&mut self, id: &ProgramId) -> Option<&mut Program> {
    self.programs.get_mut(id)
  }
}
