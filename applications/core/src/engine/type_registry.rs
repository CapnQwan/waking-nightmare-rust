use std::collections::HashMap;

pub struct TypeRegistry {
  map: HashMap<String, ComponentRegistration>,
}
