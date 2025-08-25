// vectors/vector2.rs
use super::object_traits::ObjectTraits;

struct Object {
  name: String,
  id: number,
}

impl Object {
  fn new(name, id) -> self {
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