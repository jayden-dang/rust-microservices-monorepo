use std::env;

use axum::{http::StatusCode, response::IntoResponse, Json};
use config::ConfigError;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorResponse {
  #[error("Configuration Error")]
  ConfigError(#[from] ConfigError),

  #[error("Enviroment Errors")]
  EnvError(#[from] env::VarError),
}

impl IntoResponse for ErrorResponse {
  fn into_response(self) -> axum::response::Response {
    match self {
      ErrorResponse::ConfigError(e) => {
        let body = Json(json!({"error": e.to_string(), "code": 201, "data": null, "status": false}));
        (StatusCode::BAD_REQUEST, body).into_response()
      },
      ErrorResponse::EnvError(e) => {
        let body = Json(json!({"error": e.to_string(), "code": 201, "data": null, "status": false}));
        (StatusCode::CONFLICT, body).into_response()
      },
    }
  }
}
