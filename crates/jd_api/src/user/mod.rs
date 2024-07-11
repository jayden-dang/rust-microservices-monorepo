use axum::{
  extract::{Path, State},
  routing, Json, Router,
};
use jd_core::AppResult;
use jd_domain::user::{request::RequestGetUser, User};
use sqlx::PgPool;

pub fn get_user_route() -> Router<PgPool> {
  pub async fn get_user_by_id(State(db): State<PgPool>, Path(id): Path<RequestGetUser>) -> AppResult<Json<User>> {
    jd_infra::user::get_user(State(db), Path(id)).await
  }

  Router::new().route("/user/:id", routing::get(get_user_by_id))
}
