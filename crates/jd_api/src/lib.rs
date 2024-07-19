use axum::Router;
use sqlx::PgPool;
use user::{delete_user_route, get_user_route, get_users_route, update_user_route, CourseDmc, UserDmc};

pub mod user;

pub fn user_routes() -> Router<PgPool> {
  // /api/v1/user/:id
  Router::new().nest(
    "/api/v1",
    Router::new()
      .merge(get_user_route())
      .merge(get_users_route())
      .merge(update_user_route())
      .merge(UserDmc::create_user_route())
      .merge(CourseDmc::create_course_route())
      .merge(delete_user_route()),
  )
}
