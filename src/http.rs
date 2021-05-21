use std::io::Error;
use tide::Request;

async fn index(mut req: Request<()>) -> tide::Result {
  let body = req.body_bytes().await?;

  Ok(format!("rmw body = {:?}", String::from_utf8(body)).into())
}

pub async fn listen(addr: String) -> Result<(), Error> {
  let mut app = tide::new();
  app.at("").get(index).post(index);
  app.listen(addr).await?;
  Ok(())
}
