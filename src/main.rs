use std::sync::Arc;

use axum::{routing::get, Router};
mod db;
use dotenv::dotenv;
use rust_microservice::configs::EnvDev;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use tracing::info;
pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() {
  // initialize tracing
  tracing_subscriber::fmt::init();
  dotenv().ok();

  let cfg = match EnvDev::from_env() {
    Ok(cfg) => cfg,
    Err(_) => {
      info!("Loading Env Failed");
      return;
    },
  };

  // build our application with a route
  let pool = db::intialized_db(&cfg.web.dns, cfg.web.max_conns).await;

  let app = Router::new()
    // `GET /` goes to `root`
    .route("/", get(root))
    // `POST /users` goes to `create_user`
    .with_state(Arc::new(pool));

  // run our app with hyper, listening globally on port 3000
  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
  info!("Server is running on port 3000");
  axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
  "Hello, World!"
}

// the output to our `create_user` handler
#[derive(Debug, Default, Clone, FromRow, Serialize, Deserialize)]
struct User {
  id: u64,
  username: String,
}
