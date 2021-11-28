use crate::tracker::entities::todo;
use crate::tracker::interactors::gateway::{TodoGateway, TodoMessage};

fn make_todo_message() {}

pub struct PostgresTodoGateway {}

impl TodoGateway for PostgresTodoGateway {
  fn find_all(&self) -> Vec<TodoMessage> {
    let todos = vec![
      todo::make_todo(String::from("do the dishes")),
      todo::make_todo(String::from("do the laundry")),
      todo::make_todo(String::from("write some code")),
    ];

    todos
      .into_iter()
      .map(|todo| TodoMessage {
        id: todo.id,
        label: todo.label,
        done: todo.done,
      })
      .collect()
  }
}
