mod index;
use std::io::Error;

pub async fn listen(addr: String) -> Result<(), Error> {
  let mut app = tide::new();
  app.at("").get(index::index).post(index::index);
  app.listen(addr).await?;
  Ok(())
}
