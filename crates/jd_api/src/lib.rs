use axum::Router;
use sqlx::PgPool;
mod course;
mod user;

pub fn user_routes() -> Router<PgPool> {
  Router::new().nest("/api/v1", Router::new().merge(course::routes()).merge(user::routes()))
}
