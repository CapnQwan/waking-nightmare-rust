pub struct Program {
  id: Option<u32>,
  pub program: u32,
}

impl Program {
  pub fn new(program: u32) -> Self {
    Program { id: None, program }
  }

  pub fn id(&self) -> Option<u32> {
    self.id
  }

  pub fn set_id(&mut self, id: u32) -> &mut Self {
    self.id = Some(id);
    self
  }
}
