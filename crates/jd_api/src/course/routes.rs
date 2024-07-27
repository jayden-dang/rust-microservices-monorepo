use axum::{
  routing::{get, post},
  Router,
};
use sqlx::PgPool;

use super::CourseDmc;

pub fn routes() -> Router<PgPool> {
  Router::new().route("/course", post(CourseDmc::create_course)).route("/course/:id", get(CourseDmc::get_course_by_id))
}
