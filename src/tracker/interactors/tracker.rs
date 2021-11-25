use crate::tracker::entities::todo::{self, Todo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoMessage {
  pub id: u32,
  pub label: String,
  pub done: bool,
}

fn todo_to_message(todo: Todo) -> TodoMessage {
  TodoMessage {
    id: todo.id,
    label: todo.label,
    done: todo.done,
  }
}

pub fn get_all_todos() -> Vec<TodoMessage> {
  let todos = vec![
    todo::make_todo(String::from("do the dishes")),
    todo::make_todo(String::from("do the laundry")),
    todo::make_todo(String::from("write some code")),
  ];

  let messages = todos
    .into_iter()
    .map(|todo| todo_to_message(todo))
    .collect();

  messages
}
