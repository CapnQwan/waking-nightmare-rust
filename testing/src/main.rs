use std::{any::Any, collections::HashMap, fmt::Debug};

#[derive(Clone, Debug, PartialEq, Hash)]
enum AppEvent {
  UserLoggedIn { user_id: u64 },
  UserLoggedOut { user_id: u64 },
  OrderPlaced { order_id: u64, amount: f64 },
  OrderShipped { order_id: u64, tracking: String },
}

fn main() {
  let events: HashMap<AppEvent, Box<dyn Fn(&AppEvent) + 'static>> = HashMap::new();

  events.insert();

  println!("test complete");
}
