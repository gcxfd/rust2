use crate::args::DIR;
use sled::{Db, Tree};
use static_init::dynamic;
use std::path::Path;

#[dynamic]
pub static KV: Db = sled::Config::new()
  .path(Path::new(&*DIR).join("kv"))
  .use_compression(true)
  .cache_capacity(256 * 1024 * 1024) // 256 MB
  .open()
  .unwrap();

#[dynamic]
pub static ipv4: Tree = KV.open_tree("ipv4").unwrap();

#[dynamic]
pub static id: Tree = KV.open_tree("id").unwrap();
