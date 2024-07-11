use serde::Serialize;
use sqlx::prelude::FromRow;

pub mod request;
pub mod response;

#[derive(Serialize, FromRow)]
pub struct User {
  pub pk_user_id: i64,
  pub username: String,
}
