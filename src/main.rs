use std::{env, sync::Arc};

use axum::{
  extract::Path,
  http::StatusCode,
  response::{IntoResponse, Response},
  routing::get,
  Extension, Json, Router,
};
use dotenv::{dotenv, var};
use rust_microservice::{configs::ProdConfig, dbs::initialed_db, errors::AppError, AppResult};
use serde_json::json;
use tracing::info;

#[tokio::main]
async fn main() {
  dotenv().ok();
  tracing_subscriber::fmt::init();

  let cfg = ProdConfig::from_env().expect("Cann't get env");
  let pool = initialed_db(&cfg.postgres.dsn, cfg.postgres.max_conns).await;

  let app = Router::new().route("/:msg", get(say_hello)).layer(Extension(Arc::new(pool)));
  info!("Connect Database successfully");

  info!("Server is running on port: {}", cfg.web.addr);
  let listener = tokio::net::TcpListener::bind(cfg.web.addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

pub async fn say_hello(Path(msg): Path<String>) -> AppResult<Json<serde_json::Value>> {
  if !msg.is_empty() {
    Err(AppError::NotFound)
  } else {
    Ok(Json(json!({"msg" : msg})))
  }
}
