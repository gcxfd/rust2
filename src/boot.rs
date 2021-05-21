use crate::{http, udp, ws};

use crate::config_get;

use log::info;
use rand::Rng;

macro_rules! listen {
  ($func:ident, $method:ident, $default: block) => {{
    let addr = config_get!($method, $func, $default);
    info!("{}://{}", stringify!($func), addr);
    $func::listen(addr)
  }};
}

pub async fn boot() {
  info!("> {:?}", std::env::current_exe().unwrap().parent().unwrap());
  let _ = futures::join!(
    listen!(udp, create, {
      format!("0.0.0.0:{}", rand::thread_rng().gen_range(10000..30000))
    }),
    listen!(ws, default, { "0.0.0.0:9980".to_string() }),
    listen!(http, default, { "0.0.0.0:9981".to_string() }),
  );
}
