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

// @todo - updating / replacing a value at index x?
// @todo - interating over dense
// @todo - iterating with threading?

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

  pub fn extract(&mut self, sparse_index: usize) -> Option<T> {
    if self.dense.is_empty() || sparse_index >= self.sparse.len() {
      return None;
    };

    let last_items_index = self.dense.len() - 1;
    // last_items_sparse_index = 16
    let Some(last_items_sparse_index) = self.inverse.get(last_items_index) else {
      return None;
    };
    // dense_index = 3
    let Some(dense_index) = self.sparse.get(sparse_index).and_then(|opt| opt.as_ref()) else {
      return None;
    };

    self.dense.swap(*dense_index, last_items_index);
    self.inverse.swap(*dense_index, last_items_index);

    self.sparse[*last_items_sparse_index] = Some(*dense_index);
    self.sparse[sparse_index] = None;
    self.dense.pop()
  }

  pub fn replace(&mut self, sparse_index: usize, value: T) {
    if self.dense.is_empty() || sparse_index >= self.sparse.len() {
      return;
    };

    self
      .sparse
      .get(sparse_index)
      .and_then(|opt| opt.as_ref())
      .and_then(|&dense_index| Some(self.dense[dense_index] = value));
  }
}
