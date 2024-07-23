use axum::{routing::post, Router};
use sqlx::PgPool;

use super::CourseDmc;

pub fn routes() -> Router<PgPool> {
  Router::new().route("/course", post(CourseDmc::create_course))
}
