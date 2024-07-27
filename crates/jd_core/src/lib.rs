use error::AppError;
use sea_query::Iden;

pub mod config;
pub mod error;

pub type AppResult<T> = std::result::Result<T, AppError>;
