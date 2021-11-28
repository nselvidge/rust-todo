mod tracker;

use tracker::adapters::Adapter;
use tracker::gateways::todo::PostgresTodoGateway;

#[async_std::main]
async fn main() -> tide::Result<()> {
  let mut app = tide::new();
  let adapters = Adapter::new(PostgresTodoGateway {});

  adapters.add_routes(&mut app);
  app.listen("127.0.0.1:8081").await?;
  Ok(())
}
