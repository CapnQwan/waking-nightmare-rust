// A small, generic trait for implementing a registry / container that
// stores items of type `T` and returns identifiers of type `ID` to look
// them up later.
//
// @Note
// Add deletion logic? if neccarsary
//
// @Note
// Something to keep in mind if adding deletion logic maybe adding
// some sort of implementation for reusing IDs of deleted items once
// they have been free
//
pub trait Registry<ID, T> {
  fn new() -> Self;
  fn register(&mut self, item: T) -> ID;
  fn get(&self, id: &ID) -> Option<&T>;
  fn get_mut(&mut self, id: &ID) -> Option<&mut T>;
}
