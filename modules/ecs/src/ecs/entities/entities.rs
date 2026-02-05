#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entity(pub usize);

impl Entity {
  pub fn id(&self) -> usize {
    self.0
  }
}
