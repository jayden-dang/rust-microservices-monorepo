use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestGetUser {
  pub id: i64,
}

#[derive(Deserialize)]
pub struct RequestCreateUser {
  pub username: String,
}

#[derive(Deserialize)]
pub struct RequestUpdateUser {
  pub id: i64,
  pub username: String,
}
