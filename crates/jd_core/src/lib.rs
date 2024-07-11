use error::AppError;

pub mod config;
pub mod error;

pub type AppResult<T> = std::result::Result<T, AppError>;
