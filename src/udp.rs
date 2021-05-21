use crate::encode;
use crate::file::test;
use crate::net::addr_to_bytes::ToBytes;
use async_std::net::UdpSocket;
use bytes::BytesMut;
use log::{error, info};
use std::io::Error;

pub const MTU: usize = 1472;

pub async fn listen(addr: String) -> Result<(), Error> {
  encode::u64();

  match test() {
    Ok(_) => {
      info!("test ok");
    }
    Err(err) => {
      error!("{:?}", err);
    }
  }

  let socket = UdpSocket::bind(addr).await?;

  macro_rules! send_to {
    ($val:expr, $addr:expr) => {
      Await!(socket.send_to(&$val, $addr));
    };
  }

  let mut input = BytesMut::new();
  input.resize(MTU, 0);
  loop {
    match socket.recv_from(&mut input).await {
      Err(err) => error!("{:?}", err),
      Ok((n, src)) => {
        macro_rules! reply {
          ($val:expr) => {
            send_to!($val, src);
          };
        }
        if n == 0 {
          println!("{} >", src);
        } else {
          println!("{} {:?} > {}", src, src.to_bytes(), input[0]);
          reply!([]);
        }
      }
    }
  }
}
