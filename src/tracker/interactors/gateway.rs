use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoMessage {
  pub id: u32,
  pub label: String,
  pub done: bool,
}

pub trait TodoGateway {
  fn find_all(&self) -> Vec<TodoMessage>;
}

pub struct Gateways<T: TodoGateway> {
  todos: T,
}
