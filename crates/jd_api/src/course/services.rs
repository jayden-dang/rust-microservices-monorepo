use axum::{extract::State, Json};
use jd_core::AppResult;
use jd_domain::user::{RequestCreateCourse, ResponseCreateCourse};
use sqlx::PgPool;

use super::CourseDmc;

impl CourseDmc {
  pub async fn create_course(
    State(db): State<PgPool>,
    Json(req): Json<RequestCreateCourse>,
  ) -> AppResult<Json<ResponseCreateCourse>> {
    jd_infra::user::create::<CourseDmc, _, _>(db, req).await
  }
}
