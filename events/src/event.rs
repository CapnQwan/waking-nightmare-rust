use std::cell::RefCell;
use std::sync::Arc;

pub struct Event<T: Clone> {
  // `Arc` for thread-safe shared ownership,
  // `RefCell` for interior mutability.
  subscribers: Arc<RefCell<Vec<Box<dyn Fn(&T) + Send + Sync + 'static>>>>,
}

impl<T: Clone> Event<T> {
  fn new() -> Self {
    Event {
      subscribers: Arc::new(RefCell::new(Vec::new())),
    }
  }

  pub fn subscribe<F>(&self, callback: F)
  where
    F: Fn(&T) + Send + Sync + 'static,
  {
    self.subscribers.borrow_mut().push(Box::new(callback));
  }

  pub fn unsubscribe<F>() {}

  pub fn invoke(&self, data: &T) {
    let borrow = self.subscribers.borrow();
    let callbacks: Vec<_> = borrow.iter().collect();

    for cb in callbacks {
      cb(data.clone());
    }
  }
}
