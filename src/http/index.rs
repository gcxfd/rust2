use crate::kv;
use db::DB;
use sqlx::query;
use std::convert::TryInto;
use tide::{Body, Request, Response};

fn increment(old: Option<&[u8]>) -> Option<Vec<u8>> {
  let number = match old {
    Some(bytes) => {
      let array: [u8; 8] = bytes.try_into().unwrap();
      let number = u64::from_be_bytes(array);
      number + 1
    }
    None => 1,
  };

  Some(number.to_be_bytes().to_vec())
}

pub async fn post(_: Request<()>) -> tide::Result {
  Ok(match kv::id.update_and_fetch("ipv4", increment)? {
    Some(id) => Body::from_bytes(id.to_vec()).into(),
    _ => Response::new(500),
  })
}

pub async fn get(mut req: Request<()>) -> tide::Result {
  let body = req.body_bytes().await?;
  println!("body = {:?}", body);
  println!(
    "sqlite version = {:?}",
    query!("select sqlite_version();")
      .fetch_all(&DB.read())
      .await?
  );

  Ok(match kv::id.update_and_fetch("ipv4", increment)? {
    Some(id) => format!("ipv4 id {}", u64::from_be_bytes((&*id).try_into()?)).into(),
    _ => Response::new(500),
  })
}
