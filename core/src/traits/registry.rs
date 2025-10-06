pub trait Registry<ID, T> {
  fn new() -> Self;
  fn register(&mut self, item: T) -> ID;
  fn get(&self, id: &ID) -> Option<&T>;
  fn get_mut(&mut self, id: &ID) -> Option<&mut T>;
}
