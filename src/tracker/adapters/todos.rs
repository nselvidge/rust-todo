use crate::tracker::interactors::gateway::TodoGateway;
use crate::tracker::interactors::tracker::TrackerInteractor;
use std::sync::Arc;
use tide::{Request, Result, Server};

pub struct TodoAdapter<T: TodoGateway> {
  tracker_interactor: TrackerInteractor<T>,
}

impl<T: TodoGateway + Send + Sync> TodoAdapter<T> {
  pub fn new(todo_gateway: T) -> TodoAdapter<T> {
    TodoAdapter {
      tracker_interactor: TrackerInteractor::new(todo_gateway),
    }
  }

  async fn list_todos(&self, mut _req: Request<()>) -> Result {
    let todos = self.tracker_interactor.get_all_todos();
    Ok(
      serde_json::to_string(&todos)
        .expect("Error serializing todos into json")
        .into(),
    )
  }

  pub fn add_routes(&self, app: &mut Server<()>) {
    let adapter = Arc::new(self);
    let copy = adapter.clone();
    app
      .at("/todos")
      .get(|mut req| async move { copy.list_todos(req).await });
  }
}
