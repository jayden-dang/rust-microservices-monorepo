pub mod middleware;
pub mod user;

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn initialed_db(dsn: &str, max_conns: u32) -> PgPool {
  let db = PgPoolOptions::new().max_connections(max_conns).connect(dsn).await.expect("Cannot connect database");

  sqlx::migrate!().run(&db).await.expect("Cannot migrate database");

  db
}
