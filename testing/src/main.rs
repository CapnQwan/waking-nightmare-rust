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

//
// HashMap of AppEvent
//
// UserLoggedIn - Vec<> of { user_id }
// UserLoggedOut - Vec<> of { user_id }
// OrderPlaced - Vec<> of { order_id, amount }
// OrderShipped - Vec<> of { user_id }
//

// fn main() {
//   let enum_val_a = AppEvent::UserLoggedIn { user_id: 1 };
//   let type_id_a = enum_val_a.type_id();

//   println!("type id a - {:?}", type_id_a);

//   let drink = Drinks::Tea;

//   println!("drink - {}", drink.);

//   println!("test complete");
// }
