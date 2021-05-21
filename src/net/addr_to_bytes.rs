use bytes::{BufMut, Bytes, BytesMut};
use std::net::{IpAddr, SocketAddr};

pub trait ToBytes {
  fn to_bytes(&self) -> Bytes;
}

impl ToBytes for SocketAddr {
  fn to_bytes(&self) -> Bytes {
    let mut r = BytesMut::with_capacity(16 + 2);
    match self.ip() {
      IpAddr::V4(ip) => {
        r.put_u32(ip.into());
      }
      IpAddr::V6(ip) => {
        r.put_u128(ip.into());
      }
    }
    r.put_u16(self.port());
    r.into()
  }
}
