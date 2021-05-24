use crate::args::DIR;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::{Connection, SqliteConnection};
use static_init::dynamic;
use std::path::Path;
use std::str::FromStr;

#[dynamic]
pub static mut DB: SqliteConnection = futures::executor::block_on(async {
  SqliteConnection::connect_with(
    &SqliteConnectOptions::from_str(&format!(
      "sqlite://{}",
      Path::new(&*DIR).join("db.sqlite").display().to_string()
    ))
    .unwrap()
    .synchronous(SqliteSynchronous::Normal)
    .create_if_missing(true)
    .journal_mode(SqliteJournalMode::Wal),
  )
  .await
  .unwrap()
});
