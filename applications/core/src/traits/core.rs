
use crate::traits::query::Query;
use crate::traits::command::Command;

pub trait Updatable {
  fn update(&mut self);
}

pub trait Commandable {
  fn handle_command(&mut self, command: dyn Command);
}

pub trait Queryable {
  fn handle_query(&mut self, query: dyn Query);
}