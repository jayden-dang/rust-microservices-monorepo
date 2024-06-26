use std::{env, error::Error};

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("Configuration Error")]
  Config(#[from] config::ConfigError),

  #[error("Not found...")]
  NotFound,

  #[error("Env Error")]
  EnvError(#[from] env::VarError),
}

pub struct WebResponse {
  pub data: Value,
  pub code: StatusCode,
  pub status: bool,
}

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
    match self {
      AppError::Config(e) => (StatusCode::BAD_REQUEST, Json(json!({"error" : e.to_string()}))).into_response(),
      AppError::NotFound => (StatusCode::NOT_FOUND, Json(json!({"error": "Not Found"}))).into_response(),
      AppError::EnvError(e) => (StatusCode::FORBIDDEN, Json(json!({"error": e.to_string()}))).into_response(),
    }
  }
}
