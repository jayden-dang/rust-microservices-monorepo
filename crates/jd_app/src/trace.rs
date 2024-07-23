use chrono::Local;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Registry};

// TODO: telemetry -> Tracing microservices
pub fn tracing_init() -> WorkerGuard {
  let format_layer = fmt::layer().pretty().with_writer(std::io::stderr);
  let log_file_name = Local::now().format("%Y-%m-%d").to_string() + ".log";

  let file_appender = rolling::daily("logs", log_file_name);
  let (non_blocking_appender, gaurd) = tracing_appender::non_blocking(file_appender);

  let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking_appender);

  Registry::default().with(format_layer).with(file_layer).init();
  gaurd
}
