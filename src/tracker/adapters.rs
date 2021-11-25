mod todos;

use tide::Server;

pub fn add_routes(app: &mut Server<()>) {
  todos::add_routes(app);
}
