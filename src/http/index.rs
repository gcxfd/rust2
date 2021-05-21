use crate::kv;
use std::convert::TryInto;
use tide::{Body, Request};

pub async fn post(mut req: Request<()>) -> tide::Result {
  let id = (match kv::id.get(b"ipv4")? {
    Some(id) => u64::from_be_bytes((&*id).try_into()?),
    _ => 0,
  } + 1)
    .to_be_bytes();

  kv::id.set(b"ipv4", &id);
  Ok(Body::from_bytes(id.to_vec()).into())
}

pub async fn get(mut req: Request<()>) -> tide::Result {
  let body = req.body_bytes().await?;

  let id = match kv::id.get(b"ipv4")? {
    Some(id) => u64::from_be_bytes((&*id).try_into()?),
    _ => 0,
  };

  Ok(format!("ipv4 id {}", id).into())
}
