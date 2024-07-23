use axum::{
  routing::{get, patch, post},
  Router,
};
use sqlx::PgPool;

use super::UserDmc;

pub fn routes() -> Router<PgPool> {
  Router::new()
    .route("/user", post(UserDmc::create_user)) // login, admin - DCL
    .route("/user/:id", get(UserDmc::get_user_by_id).delete(UserDmc::delete_user))
    .route("/users", get(UserDmc::get_users))
    .route("/user/:id", patch(UserDmc::update_user))
}
