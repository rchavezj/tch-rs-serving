use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerCongig {
    pub port: i32,
    pub host: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
