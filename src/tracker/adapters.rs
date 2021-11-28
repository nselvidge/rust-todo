mod todos;

use super::interactors::gateway::TodoGateway;
use tide::Server;
use todos::TodoAdapter;

pub struct Adapter<T: TodoGateway> {
  todo_gateway: T,
}

impl<T: TodoGateway> Adapter<T> {
  pub fn new(todo_gateway: T) -> Adapter<T> {
    Adapter { todo_gateway }
  }
  pub fn add_routes(&self, app: &mut Server<()>) {
    let todos = TodoAdapter::new(self.todo_gateway);
    todos.add_routes(app);
  }
}
