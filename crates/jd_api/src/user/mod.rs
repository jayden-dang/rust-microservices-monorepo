use axum::{
  extract::{Path, State},
  routing, Json, Router,
};
use jd_core::AppResult;
use jd_domain::user::{
  request::{RequestCreateUser, RequestGetUser, RequestUpdateUser},
  Course, RequestCreateCourse, ResponseCreateCourse, User,
};
use jd_infra::user::DMC;
use sqlx::PgPool;

pub fn get_user_route() -> Router<PgPool> {
  pub async fn get_user_by_id(State(db): State<PgPool>, Path(id): Path<RequestGetUser>) -> AppResult<Json<User>> {
    jd_infra::user::get_user(db, id).await
  }

  Router::new().route("/user/:id", routing::get(get_user_by_id))
}

pub fn get_users_route() -> Router<PgPool> {
  pub async fn get_users(State(db): State<PgPool>) -> AppResult<Json<Vec<User>>> {
    jd_infra::user::list(db).await
  }

  Router::new().route("/users", routing::get(get_users))
}

pub fn update_user_route() -> Router<PgPool> {
  pub async fn update_user(State(db): State<PgPool>, Json(req): Json<RequestUpdateUser>) -> AppResult<()> {
    jd_infra::user::update(db, req).await
  }

  Router::new().route("/user", routing::patch(update_user))
}

pub struct UserDmc;

impl DMC for UserDmc {
  const SCHEMA: &'static str = "user";
  const TABLE: &'static str = "tbl_users";
}

impl UserDmc {
  pub fn create_user_route() -> Router<PgPool> {
    pub async fn create_user(State(db): State<PgPool>, Json(req): Json<RequestCreateUser>) -> AppResult<Json<User>> {
      jd_infra::user::create::<UserDmc, _, _>(db, req).await
    }

    Router::new().route("/user", routing::post(create_user))
  }
}

pub struct CourseDmc;

impl DMC for CourseDmc {
  const SCHEMA: &'static str = "course";
  const TABLE: &'static str = "tbl_courses";
}

impl CourseDmc {
  pub fn create_course_route() -> Router<PgPool> {
    pub async fn create_course(
      State(db): State<PgPool>,
      Json(req): Json<RequestCreateCourse>,
    ) -> AppResult<Json<ResponseCreateCourse>> {
      jd_infra::user::create::<CourseDmc, _, _>(db, req).await
    }

    Router::new().route("/course", routing::post(create_course))
  }
}

pub fn delete_user_route() -> Router<PgPool> {
  pub async fn delete_user(State(db): State<PgPool>, Path(req): Path<i64>) -> AppResult<()> {
    jd_infra::user::delete(db, req).await
  }

  Router::new().route("/user/:id", routing::delete(delete_user))
}
