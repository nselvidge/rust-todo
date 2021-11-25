mod tracker;

#[async_std::main]
async fn main() -> tide::Result<()> {
  let mut app = tide::new();
  tracker::adapters::add_routes(&mut app);
  app.listen("127.0.0.1:8081").await?;
  Ok(())
}
