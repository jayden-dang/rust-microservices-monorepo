use axum::{
  routing::{delete, get, patch, post},
  Router,
};
use sqlx::PgPool;

use super::UserDmc;

pub fn routes() -> Router<PgPool> {
  Router::new()
    .route("/user", post(UserDmc::create_user)) // login, admin - DCL
    .route("/user/:id", get(UserDmc::get_user_by_id).delete(UserDmc::delete_user).patch(UserDmc::update_user))
    .route("/users", get(UserDmc::get_users))
    .route("/users", delete(UserDmc::delete_many_user))
    .route("/user", get(UserDmc::get_user_by_sth))
    .route("/usert", get(UserDmc::get_user_sth))
    .route("/count_user", get(UserDmc::count_users))
}
