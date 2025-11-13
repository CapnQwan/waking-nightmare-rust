use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use crate::event::Event;

#[derive(Clone, Debug)]
enum AppEvent {
  UserLoggedIn { user_id: u64 },
  UserLoggedOut { user_id: u64 },
  OrderPlaced { order_id: u64, amount: f64 },
  OrderShipped { order_id: u64, tracking: String },
}

pub fn test() {
  AppEvent;
}

/// The bus itself.
#[derive(Clone, Default)]
pub struct EventBus<E: Eq + Hash + Clone> {
  inner: Arc<Mutex<EventBusInner<E>>>,
}

struct EventBusInner<E> {
  /// `event_type -> Vec<callback>`
  handlers: HashMap<E, Vec<Box<dyn Fn(&dyn E) + Send + Sync>>>,
}

impl EventBus {
  pub fn new() -> Self {
    Self::default()
  }

  /// Subscribe to a specific event variant by name (as string)
  pub fn on<F>(&self, event_name: &'static str, handler: F)
  where
    F: Fn(&AppEvent) + Send + Sync + 'static,
  {
    let mut inner = self.inner.lock().unwrap();
    inner.entry(event_name).or_default().push(Box::new(handler));
  }

  /// Publish any event
  pub fn emit(&self, event: AppEvent) {
    let event_name = event.name(); // we'll implement this
    let inner = self.inner.lock().unwrap();

    if let Some(handlers) = inner.get(event_name) {
      for handler in handlers {
        handler(&event);
      }
    }
  }
}

fn main() {
  let bus = EventBus::new();

  // Subscribe with full access to the event
  bus.on("UserLoggedIn", |event| {
    if let Some(id) = event.as_user_logged_in() {
      println!("Welcome back, user {id}!");
    }
  });

  bus.on("OrderPlaced", |event| {
    if let AppEvent::OrderPlaced { order_id, amount } = event {
      println!("Order {order_id} placed: ${amount:.2}");
    }
  });

  // Emit events
  bus.emit(AppEvent::UserLoggedIn { user_id: 123 });
  bus.emit(AppEvent::OrderPlaced {
    order_id: 7,
    amount: 99.99,
  });
}

// /** Abstraction to simplify the typing of EventCallbacks */
// pub type EventCallback<T> = dyn FnMut(&T) + Send + Sync + 'static;

// /** Unique id given to subscribers in order to unsubscribe later */
// pub type SubscriberId = usize;

// /** Event bus for handling subs and pubs of events */
// #[derive(Clone, Debug)]
// pub struct EventBus<T> {
//   events: HashMap<String, Event<>>,
// }

// impl<T> EventBus<T>
// where
//   T: Send + Sync + 'static,
// {
//   pub fn new() -> Self {
//     EventBus {
//       subscribers: ,
//     }
//   }

//   pub fn subscribe(&self) -> SubscriberId {
//     32
//   }

//   pub fn unsubscribe(&self, id: SubscriberId) {}

//   pub fn emit(&self, data: T) {}
// }
