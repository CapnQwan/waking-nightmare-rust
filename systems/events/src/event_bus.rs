use std::sync::{Arc, Mutex};

/** Abstraction to simplify the typing of EventCallbacks */
pub type EventCallback<T> = dyn FnMut(&T) + Send + Sync + 'static;

/** Unique id given to subscribers in order to unsubscribe later */
pub type SubscriberId = usize;

/** Event bus for handling subs and pubs of events */
#[derive(Clone, Debug)]
pub struct EventBus<T> {
  subscribers: Arc<Mutex<Vec<T>>>,
}

impl<T> EventBus<T>
where
  T: Send + Sync + 'static,
{
  pub fn new() -> Self {
    EventBus {
      subscribers: Arc::new(Mutex::new(Vec::new())),
    }
  }

  pub fn subscribe(&self) -> SubscriberId {
    32
  }

  pub fn unsubscribe(&self, id: SubscriberId) {}

  pub fn invoke(&self, data: T) {}
}
