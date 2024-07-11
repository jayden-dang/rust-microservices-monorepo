use serde::Serialize;

#[derive(Serialize)]
pub struct ResposeCreateUser {
  pub id: i64,
  pub username: String,
}
