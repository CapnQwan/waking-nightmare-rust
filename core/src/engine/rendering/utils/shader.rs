pub struct Shader {
  pub id: u32,
  pub program: u32,
}

impl Shader {
  pub fn new(id: u32, program: u32) -> Self {
    Shader {
      id: id,
      program: program,
    }
  }
}
