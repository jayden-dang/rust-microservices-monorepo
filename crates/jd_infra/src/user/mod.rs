use axum::Json;
use jd_core::{error::AppError, AppResult};
use jd_domain::user::{
  request::{RequestGetUser, RequestUpdateUser},
  User,
};
use modql::{field::HasFields, SIden};
use sea_query::{Alias, IntoIden, PostgresQueryBuilder, Query, TableRef};
use sea_query_binder::SqlxBinder;
use sqlx::{postgres::PgRow, FromRow, PgPool};

pub trait DMC {
  const SCHEMA: &'static str;
  const TABLE: &'static str;

  fn table_ref() -> TableRef {
    TableRef::SchemaTable(SIden(Self::SCHEMA).into_iden(), SIden(Self::TABLE).into_iden())
  }
}

// DMC -> Database Model Control
pub async fn create<MC, I, O>(db: PgPool, input: I) -> AppResult<Json<O>>
where
  MC: DMC,
  I: HasFields,
  O: HasFields + for<'a> FromRow<'a, PgRow> + Send + Unpin,
{
  // Setup Data
  let fields = input.not_none_fields();
  let (columns, sea_values) = fields.for_sea_insert();

  // Preparing Query
  let mut query = Query::insert();
  query.into_table(MC::table_ref()).columns(columns).values(sea_values)?;

  // Returning
  let o_fields = O::field_names();
  query.returning(Query::returning().columns(o_fields.iter().map(|&f| Alias::new(f)).collect::<Vec<_>>()));

  // Execute
  let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

  let entity = sqlx::query_as_with::<_, O, _>(&sql, values).fetch_one(&db).await?;

  Ok(Json(entity))
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
