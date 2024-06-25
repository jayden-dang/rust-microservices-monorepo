use std::{env, sync::Arc};

use axum::{routing::get, Extension, Json, Router};
use dotenv::dotenv;
use rust_microservice::{configs::ProdConfig, dbs::initialed_db, errors::ErrorResponse};
use serde_json::json;
use tracing::info;

#[tokio::main]
async fn main() {
  dotenv().ok();
  tracing_subscriber::fmt::init();

  let cfg = ProdConfig::from_env().expect("Cann't get env");
  let pool = initialed_db(&cfg.postgres.dsn, cfg.postgres.max_conns).await;

  let app = Router::new().route("/", get(say_hello)).layer(Extension(Arc::new(pool)));
  info!("Connect Database successfully");

  info!("Server is running on port: {}", cfg.web.addr);
  let listener = tokio::net::TcpListener::bind(cfg.web.addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

pub async fn say_hello() -> Result<Json<serde_json::Value>, ErrorResponse> {
  let result = Err(env::VarError::NotPresent);
  result.map_err(ErrorResponse::EnvError)?;

  // Nếu không có lỗi, trả về thông báo thành công
  Ok(Json(json!({"message": "Hello, world!"})))
}
