use errors::AppError;

pub mod configs;
pub mod dbs;
pub mod errors;

pub type AppResult<T> = std::result::Result<T, AppError>;
