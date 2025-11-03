pub struct Program {
  program: u32,
}

impl Program {
  pub fn new(program: u32) -> Self {
    Program { program }
  }

  pub fn program(&self) -> u32 {
    self.program
  }
}
