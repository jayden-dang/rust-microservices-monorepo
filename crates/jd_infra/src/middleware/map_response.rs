use axum::{
  http::{Method, StatusCode, Uri},
  response::{IntoResponse, Response},
};
use tracing::debug;
use uuid::Uuid;

pub async fn mw_map_response(uri: Uri, req_method: Method, res: Response) -> Response {
  let uuid = Uuid::new_v4();
  debug!("->> MAP RESPONSE");
  debug!("->> UUid: {}", uuid.to_string());
  debug!("->> Method: {}", req_method.to_string());
  debug!("->> Uri: {}", uri.to_string());
  (StatusCode::ACCEPTED, res).into_response()
}
