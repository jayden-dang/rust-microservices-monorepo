use modql::{
  field::Fields,
  filter::{FilterNodes, OpValsInt64, OpValsString},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod request;
pub mod response;

#[derive(Serialize, FromRow, Fields, Debug)]
pub struct User {
  pub pk_user_id: i64,
  pub username: String,
}

#[derive(Serialize, FromRow, Fields)]
pub struct ResponseUpdateUser {
  pub pk_user_id: i64,
}

#[derive(Serialize, FromRow, Fields)]
pub struct ResUpdateUser {
  pub pk_user_id: i64,
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

#[derive(Deserialize, FromRow, Fields)]
pub struct RequestUpdateCourse {
  pub title: String,
  pub description: String,
}

#[derive(Serialize, FromRow, Fields)]
pub struct ResponseCreateCourse {
  pub pk_course_id: i64,
  pub title: String,
}

#[derive(Deserialize, FromRow, Fields)]
pub struct RequestGetCourseById {
  pub pk_course_id: i64,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct UserFilter {
  pub pk_user_id: Option<i64>,
  pub username: Option<OpValsString>,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct CourseFilter {
  pub pk_user_id: Option<OpValsInt64>,
  pub title: Option<OpValsString>,
}
