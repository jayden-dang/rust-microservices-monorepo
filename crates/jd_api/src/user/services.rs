use axum::{
  extract::{Path, State},
  Json,
};
use jd_core::AppResult;
use jd_domain::user::{
  request::{RequestCreateUser, RequestGetUser, RequestUpdateUser},
  User,
};
use sqlx::PgPool;

use super::UserDmc;

impl UserDmc {
  pub async fn create_user(State(db): State<PgPool>, Json(req): Json<RequestCreateUser>) -> AppResult<Json<User>> {
    jd_infra::user::create::<UserDmc, _, _>(db, req).await
  }

  pub async fn get_user_by_id(State(db): State<PgPool>, Path(id): Path<RequestGetUser>) -> AppResult<Json<User>> {
    jd_infra::user::get_user(db, id).await
  }

  pub async fn get_users(State(db): State<PgPool>) -> AppResult<Json<Vec<User>>> {
    jd_infra::user::list(db).await
  }

  pub async fn update_user(State(db): State<PgPool>, Json(req): Json<RequestUpdateUser>) -> AppResult<()> {
    jd_infra::user::update(db, req).await
  }

  pub async fn delete_user(State(db): State<PgPool>, Path(req): Path<i64>) -> AppResult<()> {
    jd_infra::user::delete(db, req).await
  }
}
