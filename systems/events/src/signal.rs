use sparse_set::sparse_set::SparseSet;
use crate::event::Event;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct CallbackId(usize);

/// ## Signal
/// The Signal struct is used to create signals allowing logic to subscribe to
/// the signal and fire events which executes certain logic once an event has been invoked
///
/// ### Note
/// when adding threading support look to Arc but also look at updating the
/// emit logic to clone the callbacks and the data to prevent issues with adding to
/// the subscribers during iteration
pub struct Signal<T: Event> {
  subscribers: SparseSet<Box<dyn FnMut(&T) + 'static>>,
  free_list: Vec<usize>,
  next_id: usize,
}

impl<T: Event> Signal<T> {
  pub fn new() -> Self {
    Signal {
      subscribers: SparseSet::new(),
      free_list: Vec::new(),
      next_id: 0,
    }
  }

  pub fn subscribe<F>(&mut self, callback: F) -> CallbackId
  where
    F: FnMut(&T) + 'static,
  {
    let id = self.free_list.pop().unwrap_or_else(|| {
      let id = self.next_id;
      self.next_id += 1;
      id
    });

    self.subscribers.insert(id, Box::new(callback));
    CallbackId(id)
  }

  /// Method for unsubscribing from the listener based on the subscription id
  pub fn unsubscribe(&mut self, callback_id: CallbackId) {
    let id = callback_id.0;
    self.subscribers.extract(id);
    self.free_list.push(id);
  }

  /// Method for invoking all subscribers with the specified parameters
  ///
  /// ### Note
  /// clone is used here purely for future reference so that this supports threading
  pub fn emit(&mut self, data: &T) {
    self.subscribers.iter_mut().for_each(|callback| callback(data));
  }

  /// Removed all subscribers and resets the indices
  pub fn clear(&mut self) {
    self.subscribers.clear();
    self.free_list.clear();
    self.next_id = 0;
  }
}
