use config::ConfigError;
use dotenv::var;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Postgres {
  pub dns: String,
  pub max_conns: u32,
}

#[derive(Deserialize)]
pub struct Config {
  pub web: Postgres,
}

#[derive(Deserialize)]
pub struct DevConfig {
  pub devweb: Postgres,
}

#[derive(Deserialize)]
pub struct ProdConfig {
  pub web: Postgres,
}

#[derive(Deserialize)]
pub struct EnvDev {
  pub app: DevConfig,
}

#[derive(Deserialize)]
pub struct EnvProd {
  pub app: ProdConfig,
}

impl EnvDev {
  pub fn from_env() -> Result<Config, ConfigError> {
    match var("ENV").as_deref() {
      Ok("dev") => {
        let dev_config = config::Config::builder()
          .add_source(config::Environment::default().separator("__"))
          .build()?
          .try_deserialize::<EnvDev>()?;
        Ok(Config { web: dev_config.app.devweb })
      },
      Ok("prod") => {
        let prod_config = config::Config::builder()
          .add_source(config::Environment::default().separator("__"))
          .build()?
          .try_deserialize::<EnvProd>()?;
        Ok(Config { web: prod_config.app.web })
      },
      _ => Err(ConfigError::Message("APP_ENV environment variable not set correctly".into())),
    }
  }
}
