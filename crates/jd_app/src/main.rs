use std::sync::Arc;
use std::time::Duration;

use axum::body::Bytes;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::Extension;
use jd_core::config::ProdConfig;
use jd_infra::initialed_db;
use jd_infra::middleware::map_response::handler_404;
use jd_infra::middleware::{map_response, mw_auth};
mod trace;
use sqlx::PgPool;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tower_http::ServiceBuilderExt;
use trace::tracing_init;

use axum::{
  middleware::{self},
  Router,
};
use dotenv::dotenv;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
  pub db: PgPool,
}

impl AppState {
  pub fn new(db: PgPool) -> Arc<AppState> {
    Arc::new(Self { db })
  }
}

#[tokio::main]
async fn main() {
  dotenv().ok();
  let _gaurd = tracing_init();

  let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();

  let middleware = ServiceBuilder::new()
    .sensitive_request_headers(sensitive_headers.clone())
    .layer(
      TraceLayer::new_for_http()
        .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
          tracing::trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
        })
        .make_span_with(
          DefaultMakeSpan::new().include_headers(true)
        )
        .on_response(
          DefaultOnResponse::new()
            .include_headers(true)
            .latency_unit(tower_http::LatencyUnit::Millis)
        ))
    .sensitive_response_headers(sensitive_headers)
    .layer(TimeoutLayer::new(Duration::from_secs(10)))
    .compression()
    .insert_response_header_if_not_present(
      header::CONTENT_TYPE, HeaderValue::from_static("application/octet-stream")
    );

  let cfg = ProdConfig::from_env().expect("Cann't get env");
  let pool = initialed_db(&cfg.postgres.dsn, cfg.postgres.max_conns).await;
  let state = AppState::new(pool.clone());

  let app = Router::new()
    .merge(jd_api::user_routes())
    .layer(middleware::map_response(map_response::mw_map_response))
    .layer(middleware::from_fn_with_state(state.clone(), mw_auth::mw_auth))
    .layer(CorsLayer::new())
    .layer(middleware)
    // .layer(Extension(state))
    .fallback(handler_404)
    .with_state(pool);

  let listener = tokio::net::TcpListener::bind(cfg.web.addr).await.unwrap();
  info!("Server is running on port: {}", listener.local_addr().unwrap());
  axum::serve(listener, app).await.unwrap();
}
