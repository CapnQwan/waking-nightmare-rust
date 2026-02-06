use std::mem;

/// A sparse-set container: *O(1)* average lookup by "sparse index" with dense, contiguous storage.
///
/// This structure is commonly used in ECS implementations:
/// - **Dense storage** (`dense`) keeps all `T` values packed contiguously for cache-friendly iteration.
/// - **Sparse mapping** (`sparse`) maps an external ID (the "sparse index", e.g. an entity id)
///   to a position inside `dense`.
/// - **Inverse mapping** (`inverse`) maps from a `dense` position back to the corresponding sparse index,
///   allowing removals by swapping with the last dense element.
///
/// ## Invariants
/// For every live entry with sparse index `s`:
/// - `sparse[s] == Some(d)` where `d` is a valid index into `dense`
/// - `inverse[d] == s`
/// - `dense[d]` is the stored value for `s`
///
/// Additionally:
/// - `dense.len() == inverse.len()`
/// - `sparse.len()` may be larger than the number of stored elements (it can contain `None` holes).
///
/// ## Complexity (typical)
/// - `get`/`get_mut`: *O(1)*
/// - `insert`: amortized *O(1)* (may resize `sparse`)
/// - `extract` (remove): *O(1)* (swap-remove)
/// - Iteration: *O(n)* over stored elements in dense order
pub struct SparseSet<T> {
  /// Packed values. Indices into this vector are called **dense indices**.
  dense: Vec<T>,

  /// Maps a **sparse index** (external id) to a **dense index**.
  ///
  /// This may contain holes (`None`) when some sparse indices are unused.
  /// Example: `sparse[entity_id] -> Some(dense_index)`.
  sparse: Vec<Option<usize>>,

  /// Reverse mapping from dense index back to sparse index.
  ///
  /// This is kept in sync with `dense` so that when we swap-remove a dense element,
  /// we can update the moved element’s sparse entry in O(1).
  inverse: Vec<usize>,
}

impl<T> SparseSet<T> {
  /// Creates an empty `SparseSet`.
  pub fn new() -> Self {
    Self {
      dense: Vec::new(),
      sparse: Vec::new(),
      inverse: Vec::new(),
    }
  }

  /// Returns a shared reference to the value stored at `sparse_index`, if present.
  ///
  /// This performs bounds checks on `sparse`, then uses the mapped dense index to index `dense`.
  pub fn get(&self, sparse_index: usize) -> Option<&T> {
    self
      .sparse
      .get(sparse_index)
      .and_then(|opt| opt.as_ref())
      .and_then(|&dense_index| self.dense.get(dense_index))
  }

  /// Returns a mutable reference to the value stored at `sparse_index`, if present.
  pub fn get_mut(&mut self, sparse_index: usize) -> Option<&mut T> {
    self
      .sparse
      .get(sparse_index)
      .and_then(|opt| opt.as_ref())
      .and_then(|&dense_index| self.dense.get_mut(dense_index))
  }

  /// Inserts `value` at `sparse_index`.
  ///
  /// If `sparse_index` is beyond the end of the `sparse` vector, `sparse` is resized and
  /// missing entries are filled with `None`.
  ///
  /// Note: as written, this does **not** check for an existing element at `sparse_index`.
  /// Inserting the same `sparse_index` twice will leave an unreachable “orphan” value in `dense`
  /// (because the mapping is overwritten but the old dense slot is not removed).
  pub fn insert(&mut self, sparse_index: usize, value: T) {
    let dense_index = self.dense.len();

    if sparse_index >= self.sparse.len() {
      self.sparse.resize_with(sparse_index + 1, || None);
    }

    self.sparse[sparse_index] = Some(dense_index);
    self.inverse.push(sparse_index);
    self.dense.push(value);
  }

  /// Removes and returns the value stored at `sparse_index`, if present.
  ///
  /// Removal is done via **swap-remove**:
  /// - swap the removed element with the last element in `dense`/`inverse`
  /// - update the moved element’s sparse mapping to its new dense index
  /// - pop the last element
  ///
  /// This maintains all invariants in O(1).
  pub fn extract(&mut self, sparse_index: usize) -> Option<T> {
    if self.dense.is_empty() || sparse_index >= self.sparse.len() {
      return None;
    };

    let last_dense_index = self.dense.len() - 1;
    let last_sparse_index = self.inverse[last_dense_index];

    let Some(dense_index) = self.sparse[sparse_index] else {
      return None;
    };

    // Move the element we want to remove to the end (swap-remove).
    self.inverse.swap(dense_index, last_dense_index);
    self.dense.swap(dense_index, last_dense_index);

    // The element that used to be last is now at `dense_index`; fix its sparse mapping.
    self.sparse[last_sparse_index] = Some(dense_index);

    // The removed sparse slot is now empty.
    self.sparse[sparse_index] = None;

    // Drop the last dense/inverse entries (which now correspond to the removed element).
    self.inverse.pop();
    self.dense.pop()
  }

  /// Replaces the value at `sparse_index`, returning the old value if it existed.
  ///
  /// Note: if `sparse_index` is in-bounds but currently empty (`sparse[sparse_index] == None`),
  /// this returns `None` and does **not** insert a new value.
  ///
  /// (The doc comment now reflects the current behavior.)
  pub fn replace(&mut self, sparse_index: usize, value: T) -> Option<T> {
    if self.dense.is_empty() || sparse_index >= self.sparse.len() {
      self.insert(sparse_index, value);
      return None;
    };

    let Some(dense_index) = self.sparse[sparse_index] else {
      return None;
    };

    Some(mem::replace(&mut self.dense[dense_index], value))
  }

  /// Iterates over stored values in dense order (i.e., compact storage order).
  ///
  /// This does not yield sparse indices. If you need `(sparse_index, &T)` pairs,
  /// you’d typically iterate over `dense` indices and use `inverse[d]`.
  pub fn iter(&self) -> std::slice::Iter<'_, T> {
    self.dense.iter()
  }

  /// Mutable iteration over stored values in dense order.
  pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
    self.dense.iter_mut()
  }

  /// Removes all entries and releases their values.
  ///
  /// Note: this clears `sparse` as well, so previously-used sparse indices are forgotten.
  pub fn clear(&mut self) {
    self.dense.clear();
    self.sparse.clear();
    self.inverse.clear();
  }
}

impl<T> IntoIterator for SparseSet<T> {
  type Item = T;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  /// Consumes the set and yields values in dense order.
  fn into_iter(self) -> Self::IntoIter {
    self.dense.into_iter()
  }
}
