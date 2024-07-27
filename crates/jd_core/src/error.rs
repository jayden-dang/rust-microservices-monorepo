use std::env;
use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};
use sea_query::Iden;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("Configuration Error")]
  Config(#[from] config::ConfigError),

  #[error("Not found...")]
  NotFound,

  #[error("Env Error")]
  EnvError(#[from] env::VarError),

  #[error("Sqxl Error")]
  Sqlx(#[from] sqlx::Error),

  #[error("SeaQuery Error")]
  SeaQuery(#[from] sea_query::error::Error),

  #[error("Modql Into SeaQuery Error")]
  ModQlIntoSea(#[from] modql::filter::IntoSeaError),

  #[error("Entity Not Found")]
  EntityNotFound { entity: &'static str, id: i64 },

  #[error("Entity Not Found")]
  EntityFNotFound { entity: &'static str, fields: String },

  #[error("Limit Over Max")]
  ListLimitOverMax { max: i64, actual: i64 },

  #[error("Count Failed")]
  CountFail,

  #[error("BadRequest")]
  BadRequest(String),
}

impl IntoResponse for AppError {
  fn into_response(self) -> axum::response::Response {
    let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
    response.extensions_mut().insert(Arc::new(self));
    response
  }
}

impl AppError {
  pub fn status_and_error(&self) -> (StatusCode, ClientError) {
    use self::AppError::*;
    match self {
      EntityNotFound { entity, id } => (StatusCode::FORBIDDEN, ClientError::EntityNotFound { entity, id: *id }),
      ListLimitOverMax { max, actual } => {
        (StatusCode::FORBIDDEN, ClientError::ListLimitOverMax { max: *max, actual: *actual })
      },
      BadRequest(ref e) => (StatusCode::BAD_REQUEST, ClientError::BadRequest(e.to_string())),
      _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::ServerError),
    }
  }
}

#[derive(Serialize, Debug)]
#[serde(tag = "message", content = "details")]
pub enum ClientError {
  ServerError,
  EntityNotFound { entity: &'static str, id: i64 },
  NotFound,
  BadRequest(String),
  ListLimitOverMax { max: i64, actual: i64 },
}
