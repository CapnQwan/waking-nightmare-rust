// vectors/vector2.rs
use super::object_traits::ObjectTraits;

struct Object {
  name: String,
  id: u128,
}

impl Object {
  fn new(name: String, id: u128) -> self {
    Object { name, id }
  }
}

impl ObjectTraits for Object {
  fn instantiate(&mut self) -> Object {
    Object {      
      name: self.name,
      id: self.id,
    }
  }
}