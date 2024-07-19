use modql::field::Fields;
use serde::Deserialize;

#[derive(Deserialize, Fields)]
pub struct RequestGetUser {
  pub id: i64,
}

#[derive(Deserialize, Fields)]
pub struct RequestCreateUser {
  pub pk_user_id: i64,
  pub username: String,
}

#[derive(Deserialize, Fields)]
pub struct RequestUpdateUser {
  pub id: i64,
  pub username: String,
}
