use std::mem;

/**
 * The SparseSet is a data structure that give both the benifits of a hashmap with O(1) lookups
 * whilst also keeping the benifits of a Vec by storing all of the data contiguous in memory at
 * the cost of slighly increased size for the additional indicies vectors
 */
pub struct SparseSet<T> {
  /**
   * Dense collection of items,
   * allow for data to always be allocated next to each other in the heap
   */
  dense: Vec<T>,
  /**
   * Sparse indexing, allows for indexing to not neccararly be densly packed
   * I.E:
   *  - [0] = struct_a
   *  - [1] = None
   *  - [2] = struct_b
   */
  sparse: Vec<Option<usize>>,
  /**
   * Inverse indexing, used to store the sparse index relative to the dense index (inverse of the sparse Vec)
   * - [0] (struct_a) = 0
   * - [1] (struct_b) = 2
   */
  inverse: Vec<usize>,
}

impl<T> SparseSet<T> {
  pub fn new() -> Self {
    Self {
      dense: Vec::new(),
      sparse: Vec::new(),
      inverse: Vec::new(),
    }
  }

  pub fn get(&self, sparse_index: usize) -> Option<&T> {
    self
      .sparse
      .get(sparse_index)
      .and_then(|opt| opt.as_ref())
      .and_then(|&dense_index| self.dense.get(dense_index))
  }

  pub fn get_mut(&mut self, sparse_index: usize) -> Option<&mut T> {
    self
      .sparse
      .get(sparse_index)
      .and_then(|opt| opt.as_ref())
      .and_then(|&dense_index| self.dense.get_mut(dense_index))
  }

  pub fn insert(&mut self, sparse_index: usize, value: T) {
    let dense_index = self.dense.len();

    if sparse_index >= self.sparse.len() {
      self.sparse.resize_with(sparse_index + 1, || None);
    }

    self.sparse[sparse_index] = Some(dense_index);
    self.inverse.push(sparse_index);
    self.dense.push(value);
  }

  /**
   * Removes and returns the value at sparse_index, if it exists.
   * Maintains the sparse set invariants.
   */
  pub fn extract(&mut self, sparse_index: usize) -> Option<T> {
    if self.dense.is_empty() || sparse_index >= self.sparse.len() {
      return None;
    };

    let last_dense_index = self.dense.len() - 1;
    let last_sparse_index = self.inverse[last_dense_index];

    let Some(dense_index) = self.sparse[sparse_index] else {
      return None;
    };

    // Swap item to be removed to the back of the vectors
    self.inverse.swap(dense_index, last_dense_index);
    self.dense.swap(dense_index, last_dense_index);

    // Update the swapped item that was last in the vector to now point to it's swapped position
    self.sparse[last_sparse_index] = Some(dense_index);
    // Remove the sparse index of the removed item
    self.sparse[sparse_index] = None;

    // Remove the now last items from the vector
    self.inverse.pop();
    self.dense.pop()
  }

  /**
   * Replaces the value at sparse_index, otherwise creating the entry.
   * Returns the value that was previously stored in the dense vector
   */
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

  pub fn iter(&self) -> std::slice::Iter<'_, T> {
    self.dense.iter()
  }

  pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
    self.dense.iter_mut()
  }
}

impl IntoIterator for SparseSet<T> {
  type Item = T;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.dense.into_iter()
  }
}
