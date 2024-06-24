use config::ConfigError;
use dotenv::var;
use serde::Deserialize;

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
  pub fn from_env() -> Result<ProdConfig, ConfigError> {
    match var("ENV").as_deref() {
      Ok("prod") => {
        let config = config::Config::builder()
          .add_source(config::Environment::default())
          .build()
          .expect("Conn't loading Env")
          .try_deserialize::<ProdEnv>()?;
        Ok(ProdConfig { web: config.app.web, postgres: config.app.postgres })
      },
      _ => {
        let config = config::Config::builder()
          .add_source(config::Environment::default())
          .build()
          .expect("Conn't loading Env")
          .try_deserialize::<DevEnv>()?;
        Ok(ProdConfig { web: config.app.devweb, postgres: config.app.devpostgres })
      },
    }
  }
}
