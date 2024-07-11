use axum::{
  extract::{Path, State},
  Json,
};
use jd_core::{error::AppError, AppResult};
use jd_domain::user::{
  request::{RequestCreateUser, RequestGetUser, RequestUpdateUser},
  User,
};
use sqlx::PgPool;

pub async fn create(db: PgPool, id: RequestCreateUser) -> AppResult<Json<i64>> {
  let (id,) =
    sqlx::query_as::<_, (i64,)>(r#"INSERT INTO "user"."tbl_users" (username) VALUES ($1) returning pk_user_id"#)
      .bind(id.username)
      .fetch_optional(&db)
      .await?
      .ok_or(AppError::NotFound)?;
  Ok(Json(id))
}

pub async fn get_user(db: PgPool, id: RequestGetUser) -> AppResult<Json<User>> {
  let user: User = sqlx::query_as::<_, User>(r#"SELECT * FROM "user"."tbl_users" WHERE pk_user_id = $1"#)
    .bind(id.id)
    .fetch_optional(&db)
    .await?
    .ok_or(AppError::NotFound)?;
  Ok(Json(user))
}

pub async fn list(db: PgPool) -> AppResult<Json<Vec<User>>> {
  let user: Vec<User> =
    sqlx::query_as::<_, User>(r#"SELECT * FROM "user"."tbl_users" ORDER BY pk_user_id"#).fetch_all(&db).await?;
  Ok(Json(user))
}

pub async fn update(db: PgPool, req: RequestUpdateUser) -> AppResult<()> {
  let count = sqlx::query(r#"UPDATE "user"."tbl_users" SET username = $2 WHERE pk_user_id = $1"#)
    .bind(req.id)
    .bind(req.username)
    .execute(&db)
    .await?
    .rows_affected();
  if count == 0 {
    return Err(AppError::NotFound);
  }

  Ok(())
}

pub async fn delete(db: PgPool, id: i64) -> AppResult<()> {
  let count =
    sqlx::query(r#"DELETE FROM "user"."tbl_users" WHERE pk_user_id = $1"#).bind(id).execute(&db).await?.rows_affected();

  if count == 0 {
    return Err(AppError::NotFound);
  }

  Ok(())
}
