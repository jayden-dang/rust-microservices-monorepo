use axum::{
  extract::{Path, State},
  routing, Json, Router,
};
use jd_core::AppResult;
use jd_domain::user::{
  request::{RequestCreateUser, RequestGetUser, RequestUpdateUser},
  User,
};
use sqlx::PgPool;

pub fn get_user_route() -> Router<PgPool> {
  pub async fn get_user_by_id(State(db): State<PgPool>, Path(id): Path<RequestGetUser>) -> AppResult<Json<User>> {
    jd_infra::user::get_user(db, id).await
  }

  Router::new().route("/user/:id", routing::get(get_user_by_id))
}

pub fn get_users_route() -> Router<PgPool> {
  pub async fn get_users(State(db): State<PgPool>) -> AppResult<Json<Vec<User>>> {
    jd_infra::user::list(db).await
  }

  Router::new().route("/users", routing::get(get_users))
}

pub fn update_user_route() -> Router<PgPool> {
  pub async fn update_user(State(db): State<PgPool>, Json(req): Json<RequestUpdateUser>) -> AppResult<()> {
    jd_infra::user::update(db, req).await
  }

  Router::new().route("/user", routing::patch(update_user))
}

pub fn create_user_route() -> Router<PgPool> {
  pub async fn create_user(State(db): State<PgPool>, Json(req): Json<RequestCreateUser>) -> AppResult<Json<i64>> {
    jd_infra::user::create(db, req).await
  }

  Router::new().route("/user", routing::post(create_user))
}

pub fn delete_user_route() -> Router<PgPool> {
  pub async fn delete_user(State(db): State<PgPool>, Path(req): Path<i64>) -> AppResult<()> {
    jd_infra::user::delete(db, req).await
  }

  Router::new().route("/user/:id", routing::delete(delete_user))
}
