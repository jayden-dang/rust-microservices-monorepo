use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestGetUser {
  pub id: i64,
}

#[derive(Deserialize)]
pub struct RequestCreateUser {
  pub username: String,
  pub password: String,
  pub full_name: Option<String>,
}
