use crate::tracker::interactors::tracker;
use tide::{Request, Result, Server};

async fn list_todos(mut _req: Request<()>) -> Result {
  let todos = tracker::get_all_todos();
  Ok(
    serde_json::to_string(&todos)
      .expect("Error serializing todos into json")
      .into(),
  )
}

pub fn add_routes(app: &mut Server<()>) {
  app.at("/todos").get(list_todos);
}
