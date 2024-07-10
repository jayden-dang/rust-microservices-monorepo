use dotenv::var;
use serde::Deserialize;

use crate::{error::AppError, AppResult};

#[derive(Deserialize)]
pub struct WebConfig {
  pub addr: String,
}

#[derive(Deserialize)]
pub struct Postgres {
  pub dsn: String,
  pub max_conns: u32,
}

// Env Prod
#[derive(Deserialize)]
pub struct ProdConfig {
  pub web: WebConfig,
  pub postgres: Postgres,
}

// Env Dev
#[derive(Deserialize)]
pub struct DevConfig {
  pub devweb: WebConfig,
  pub devpostgres: Postgres,
}

// Wrap
#[derive(Deserialize)]
pub struct DevEnv {
  pub app: DevConfig,
}

// Wrap
#[derive(Deserialize)]
pub struct ProdEnv {
  pub app: ProdConfig,
}

impl ProdConfig {
  pub fn from_env() -> AppResult<ProdConfig> {
    match var("ENV").as_deref() {
      Ok("prod") => {
        let config = config::Config::builder()
          .add_source(config::Environment::default())
          .build()
          .map_err(AppError::Config)?
          .try_deserialize::<ProdEnv>()
          .map_err(AppError::Config)?;
        Ok(ProdConfig { web: config.app.web, postgres: config.app.postgres })
      },
      _ => {
        let config = config::Config::builder()
          .add_source(config::Environment::default())
          .build()
          .map_err(AppError::Config)?
          .try_deserialize::<DevEnv>()
          .map_err(AppError::Config)?;
        Ok(ProdConfig { web: config.app.devweb, postgres: config.app.devpostgres })
      },
    }
  }
}
