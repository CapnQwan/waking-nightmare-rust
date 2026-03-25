pub trait Component: Send + Sync {
  fn serialize(&self) -> String;
}

pub trait ComponentMetaData: Send + Sync {
  fn type_name() -> &'static str;
  fn deserialize(ron: &str) -> Box<dyn Component>;
}
