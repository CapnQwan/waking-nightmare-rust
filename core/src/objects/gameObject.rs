struct GameObject {
  name: String,
  id: number,
  //transform: transform,
  parent: gameObject,
  //components: component[],
  //rendererComponents: rendererComponent,
}

impl ObjectTraits for GameObject {
  fn instantiate(&mut self) -> Object {
    Object {      
      name: self.name,
      id: self.id,
    }
  }
}