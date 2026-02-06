/// An `Entity` is a lightweight handle (ID) used to index component storages.
///
/// Notes:
/// - This is currently just a numeric ID. If you recycle IDs (you do), then old
///   `Entity` values can become "stale" and accidentally refer to a newer entity
///   with the same ID.
/// - The typical fix is to add a "generation" counter (index + generation).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entity(pub usize);

impl Entity {
  /// Returns the numeric entity ID.
  ///
  /// This ID is used as the index into sparse-set component storages.
  #[inline]
  pub fn id(&self) -> usize {
    self.0
  }
}
