#[macro_use]
mod r#macro;

mod args;
mod boot;
mod config;
mod encode;
mod file;
mod http;
mod init;
mod net;
mod udp;
mod var;
mod ws;

#[allow(non_upper_case_globals)]
mod kv;

#[async_std::main]
async fn main() {
  init::init();
  boot::boot().await;
}
/*
#[macro_use]
extern crate log;
#[macro_use]
#[macro_use]
mod r#macro;

use async_std::net::UdpSocket;
use failure::Error;

mod lib;
mod listen;
mod var;

use init::init;
use listen::listen;

#[async_std::main]
async fn main() -> Result<(), Error> {
  init();

  let port = config_get!(port);
  let host = config_get!(host);
  let addr = format!(
    "{}:{}",
    if host.is_empty() {
      "0.0.0.0"
    } else {
      host.as_str()
    },
    if port.is_empty() { "0" } else { port.as_str() }
  );


    match igd::search_gateway(Default::default()) {
        Err(ref err) => println!("Error: {}", err),
        Ok(gateway) => match gateway.get_external_ip() {
            Err(ref err) => {
                println!("There was an error! {}", err);
            }
            Ok(ext_addr) => {
                println!("Local gateway: {}, External ip address: {}", gateway, ext_addr);
            }
        },
    }

  let socket = UdpSocket::bind(addr).await?;
  if port.is_empty() {
    config_set!(port, socket.local_addr()?.port().to_string());
  };

  // let mut buf = BytesMut::with_capacity(4096);
  // buf.put(&b"hello 2"[..]);
  // socket.send_to(&buf.split(), "47.104.79.244:30110").await?;
  listen(socket).await;
  Ok(())
}

// mod blake3;
// use async_std::task::spawn_blocking;
// use futures::future::join_all;
//
// async fn say_hello(sleep: u64) {
//   let txt = sleep.to_string();
//   let vec = txt.as_bytes().to_vec();
//   println!("begin {}", txt);
//
//   let r = spawn_blocking(|| blake3::hash_leading_zero(vec, 18)).await;
//
//   println!("{} {:?}", txt, r);
// }
//
// #[async_std::main]
// async fn main() {
//   let mut li = vec![];
//   for n in 1..10 {
//     li.push(say_hello(n));
//   }
//   println!("join");
//   join_all(li).await;
// }
*/
