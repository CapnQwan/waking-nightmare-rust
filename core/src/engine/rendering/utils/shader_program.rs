use crate::engine::ProgramId;

pub struct Program {
  id: Option<ProgramId>,
  pub program: u32,
}

impl Program {
  pub fn new(program: u32) -> Self {
    Program { id: None, program }
  }

  pub fn id(&self) -> Option<ProgramId> {
    self.id
  }

  pub fn set_id(&mut self, id: ProgramId) -> &mut Self {
    self.id = Some(id);
    self
  }
}
