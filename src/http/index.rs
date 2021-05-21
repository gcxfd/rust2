use tide::Request;

pub async fn index(mut req: Request<()>) -> tide::Result {
  let body = req.body_bytes().await?;

  Ok(format!("rmw body {}", String::from_utf8(body)?).into())
}
