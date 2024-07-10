use core::{config::ProdConfig, error::AppError, AppResult};
use infra::initialed_db;

use axum::{
  extract::{Path, Request, State},
  http::{Method, StatusCode, Uri},
  middleware::{self, Next},
  response::{IntoResponse, Response},
  routing::get,
  Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

#[tokio::main]
async fn main() {
  dotenv().ok();
  tracing_subscriber::fmt::init();

  let cfg = ProdConfig::from_env().expect("Cann't get env");
  let pool = initialed_db(&cfg.postgres.dsn, cfg.postgres.max_conns).await;

  let app = Router::new()
    .route("/:msg", get(say_hello)) // auth
    .route("/user/:id", get(get_user))
    .layer(middleware::map_response(mw_map_response)) // 1
    .layer(middleware::from_fn_with_state(pool.clone(), mw_auth)) // 2
    .with_state(pool);

  let listener = tokio::net::TcpListener::bind(cfg.web.addr).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}

pub async fn say_hello(Path(msg): Path<String>) -> AppResult<Json<serde_json::Value>> {
  if msg.is_empty() {
    Err(AppError::NotFound)
  } else {
    Ok(Json(json!({"msg" : msg})))
  }
}

#[derive(Serialize, FromRow)]
pub struct User {
  pub pk_user_id: i64,
  pub username: String,
}

#[derive(Deserialize)]
pub struct UserId {
  pub id: i64,
}

pub async fn get_user(State(db): State<PgPool>, Path(id): Path<UserId>) -> AppResult<Json<User>> {
  let user: User = sqlx::query_as::<_, User>(r#"SELECT * FROM "user"."tbl_users" WHERE pk_user_id = $1"#)
    .bind(id.id)
    .fetch_optional(&db)
    .await?
    .ok_or(AppError::NotFound)?;
  Ok(Json(user))
}

pub async fn mw_map_response(uri: Uri, req_method: Method, res: Response) -> Response {
  let uuid = Uuid::new_v4();
  (StatusCode::ACCEPTED, res).into_response()
}

pub async fn mw_auth(req: Request, next: Next) -> AppResult<Response> {
  Ok(next.run(req).await)
}
