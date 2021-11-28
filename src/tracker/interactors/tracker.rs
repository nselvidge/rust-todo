use crate::tracker::entities::todo::Todo;
use crate::tracker::interactors::gateway::{TodoGateway, TodoMessage};

fn todo_to_message(todo: Todo) -> TodoMessage {
  TodoMessage {
    id: todo.id,
    label: todo.label,
    done: todo.done,
  }
}

pub struct TrackerInteractor<T: TodoGateway> {
  todo_gateway: T,
}

impl<T: TodoGateway> TrackerInteractor<T> {
  pub fn new(todo_gateway: T) -> TrackerInteractor<T> {
    TrackerInteractor { todo_gateway }
  }
  pub fn get_all_todos(&self) -> Vec<TodoMessage> {
    self.todo_gateway.find_all()
  }
}
