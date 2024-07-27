use axum::{
  extract::{Path, Query, State},
  Json,
};
use jd_core::{error::AppError, AppResult};
use jd_domain::user::{
  request::{RequestCreateUser, RequestGetUser, RequestUpdateUser},
  ResponseUpdateUser, User, UserFilter,
};
use modql::filter::ListOptions;
use serde_json::{json, Value};
use sqlx::PgPool;

use super::UserDmc;

impl UserDmc {
  pub async fn create_user(State(db): State<PgPool>, Json(req): Json<RequestCreateUser>) -> AppResult<Json<User>> {
    let user = jd_infra::user::create::<UserDmc, _, _>(db, req).await?;
    Ok(Json(user))
  }

  pub async fn get_user_by_id(Path(id): Path<i64>, State(db): State<PgPool>) -> AppResult<Json<User>> {
    let user = jd_infra::user::get_by_id::<UserDmc, _>(&db, id).await?;
    Ok(Json(user))
  }

  pub async fn get_user_by_sth(
    Query(query): Query<UserFilter>,
    Query(list_options): Query<ListOptions>,
    State(db): State<PgPool>,
  ) -> AppResult<Json<Option<User>>> {
    if query.pk_user_id.is_none() && query.username.is_none() {
      return Err(jd_core::error::AppError::BadRequest("At least one filter criteria must be provided".to_string()));
    }
    let user = jd_infra::user::first::<UserDmc, _, _>(&db, Some(query), Some(list_options)).await?;
    Ok(Json(user))
  }

  pub async fn get_user_sth(Query(query): Query<UserFilter>, State(db): State<PgPool>) -> AppResult<Json<User>> {
    if query.pk_user_id.is_none() && query.username.is_none() {
      return Err(jd_core::error::AppError::BadRequest("At least one filter criteria must be provided".to_string()));
    }
    let user = jd_infra::user::get_by_sth::<UserDmc, _, _>(&db, Some(query)).await?;
    Ok(Json(user))
  }

  pub async fn get_users(
    Query(query): Query<UserFilter>,
    Query(list_options): Query<ListOptions>,
    State(db): State<PgPool>,
  ) -> AppResult<Json<Value>> {
    let (users, pagination) = jd_infra::user::list::<UserDmc, _, User>(&db, Some(query), Some(list_options)).await?;

    let response = json!({
        "data": users,
        "metadata": pagination
    });
    Ok(Json(response))
  }

  pub async fn count_users(Query(query): Query<UserFilter>, State(db): State<PgPool>) -> AppResult<Json<i64>> {
    let count = jd_infra::user::count::<UserDmc, _>(&db, Some(query)).await?;

    Ok(Json(count))
  }

  pub async fn update_user(
    Path(id): Path<i64>,
    State(db): State<PgPool>,
    Json(req): Json<RequestUpdateUser>,
  ) -> AppResult<()> {
    jd_infra::user::update::<UserDmc, _>(&db, id, req).await
  }

  pub async fn delete_user(State(db): State<PgPool>, Path(req): Path<i64>) -> AppResult<()> {
    jd_infra::user::delete::<UserDmc>(&db, req).await
  }

  pub async fn delete_many_user(State(db): State<PgPool>, Json(req): Json<Vec<i64>>) -> AppResult<()> {
    jd_infra::user::delete_many::<UserDmc>(&db, req).await
  }
}
