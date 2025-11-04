pub struct SparseSet<T> {
  dense: Vec<T>,
  sparse: Vec<Option<usize>>,
}

// @todo - updating / replacing a value at index x?
// @todo - interating over dense
// @todo - iterating with threading?
// @todo - getting items (non mut and mut)

impl<T> SparseSet<T> {
  pub fn new() -> Self {
    Self {
      dense: Vec::new(),
      sparse: Vec::new(),
    }
  }

  pub fn get() {}

  pub fn get_mut() {}

  pub fn insert(&mut self, sparse_index: usize, data: T) {
    let dense_index = self.dense.len();

    if sparse_index >= self.sparse.len() {
      self.sparse.resize_with(sparse_index + 1, || None);
    }

    self.sparse[sparse_index] = Some(dense_index);
    self.dense.push(data);
  }

  pub fn extract(&mut self, sparse_index: usize) -> Option<T> {
    if self.dense.len() <= 0 {
      return None;
    };

    let Some(dense_index) = self.sparse[sparse_index] else {
      return None;
    };

    let last_items_index = self.dense.len() - 1;

    self.sparse[sparse_index] = None;
    self.dense.swap(dense_index, last_items_index);
    self.dense.pop()
  }
}
