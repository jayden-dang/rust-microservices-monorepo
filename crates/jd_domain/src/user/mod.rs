use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod request;
pub mod response;

#[derive(Serialize, FromRow, Fields)]
pub struct User {
  pub pk_user_id: i64,
  pub username: String,
}

#[derive(Serialize, FromRow, Fields)]
pub struct Course {
  pub pk_course_id: i64,
  pub title: String,
  pub description: String,
}

#[derive(Deserialize, FromRow, Fields)]
pub struct RequestCreateCourse {
  pub pk_course_id: i64,
  pub title: String,
  pub description: String,
}

#[derive(Serialize, FromRow, Fields)]
pub struct ResponseCreateCourse {
  pub pk_course_id: i64,
  pub title: String,
}
