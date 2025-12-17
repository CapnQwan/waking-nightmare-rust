use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::event::Event;
use crate::signal::Signal;

pub struct EventBus {
  signals: HashMap<TypeId, Box<dyn Any>>,
}

impl EventBus {
  pub fn new() -> Self {
    Self {
      signals: HashMap::new(),
    }
  }

  pub fn signal<T: Event + 'static>(&mut self) -> &mut Signal<T> {
    self.signals
        .entry(TypeId::of::<T>())
        .or_insert_with(|| Box::new(Signal::<T>::new()))
        .downcast_mut::<Signal<T>>()
        .unwrap()
  }

  pub fn emit<T: Event + 'static>(&mut self, event: &T) {
    if let Some(signal) = self.signals.get_mut(&TypeId::of::<T>()) {
      signal
          .downcast_mut::<Signal<T>>()
          .unwrap()
          .emit(event);
    }
  }
}
