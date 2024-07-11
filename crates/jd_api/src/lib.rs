use axum::Router;
use sqlx::PgPool;
use user::get_user_route;

pub mod user;

pub fn user_routes() -> Router<PgPool> {
  // /api/v1/user/:id
  Router::new().nest("/api/v1", Router::new().merge(get_user_route()))
}
