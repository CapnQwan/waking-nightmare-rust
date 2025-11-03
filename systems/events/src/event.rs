/**
 * The Event struct is used to create events allowing logic to subscribe to
 * the event and fire certain logic once the event has been invoked
 *
 * @note - currently this is a simple implementation inteded for one way communication
 * (I.E. child listening to parent events) but if it's required to instantiate Events to
 * share logic across several systems for two way communication
 * (I.E. parent listening to child events, child listening to parent events)
 * Then subscribers will need to be wrapped in Arc<Refcell>
 */
pub struct Event<T: Clone> {
  subscribers: Vec<Box<dyn Fn(&T) + 'static>>,
}

impl<T: Clone> Event<T> {
  fn new() -> Self {
    Event {
      subscribers: Vec::new(),
    }
  }

  pub fn subscribe<F>(&mut self, callback: F)
  where
    F: Fn(&T) + 'static,
  {
    self.subscribers.push(Box::new(callback));
  }

  pub fn unsubscribe<F>() {}

  pub fn invoke(&mut self, data: &T) {
    let callbacks = self.subscribers.iter().cloned();

    for cb in callbacks {
      cb(data.clone());
    }
  }
}
