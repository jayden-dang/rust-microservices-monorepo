use axum::{
  extract::{Path, State},
  Json,
};
use jd_core::AppResult;
use jd_domain::user::{Course, RequestCreateCourse, RequestGetCourseById, ResponseCreateCourse};
use sqlx::PgPool;

use super::CourseDmc;

impl CourseDmc {
  pub async fn create_course(
    State(db): State<PgPool>,
    Json(req): Json<RequestCreateCourse>,
  ) -> AppResult<Json<ResponseCreateCourse>> {
    let course = jd_infra::user::create::<CourseDmc, _, _>(db, req).await?;
    Ok(Json(course))
  }

  pub async fn get_course_by_id(State(db): State<PgPool>, Path(id): Path<i64>) -> AppResult<Json<Course>> {
    let course = jd_infra::user::get_by_id::<CourseDmc, Course>(&db, id).await?;
    Ok(Json(course))
  }
}
