use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use rust_microservice::dbs::initialed_db;
use tracing::info;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();
  let pool = initialed_db("postgres://postgres:password@localhost:5432/postgres", 5).await;

  let app = Router::new().route("/", get(|| async { "Hello world!" })).layer(Extension(Arc::new(pool)));
  info!("Connect Database successfully");

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  info!("Server is running on port 3000");
  axum::serve(listener, app).await.unwrap();
}
