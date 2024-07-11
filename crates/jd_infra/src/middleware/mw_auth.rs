use axum::{extract::Request, middleware::Next, response::Response};
use jd_core::AppResult;
use tracing::debug;

pub async fn mw_auth(req: Request, next: Next) -> AppResult<Response> {
  debug!("->> MIDDLEWARE AUTH");
  Ok(next.run(req).await)
}
