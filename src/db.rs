use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn intialized_db(dsn: &str, max_conns: u32) -> Pool<Postgres> {
  let db = PgPoolOptions::new().max_connections(max_conns).connect(dsn).await.unwrap();

  sqlx::migrate!().run(&db).await.expect("Cannot run migrations");

  db
}
