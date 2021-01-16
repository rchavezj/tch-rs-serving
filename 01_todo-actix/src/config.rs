pub use config::ConfigError;
use serde::Deserialize;
use slog::{o, Drain, Logger};
use slog_async;
use slog_envlogger;
use slog_term;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: i32,
    pub host: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {

    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }

    // pub fn configure_pool(&self) -> Pool {
    //     self.pg.create_pool(NoTls).unwrap()
    // }

    pub fn configure_log() -> Logger {
        // // Terminal Output (Design pattern): Nice format for the logger
        // let decorator = slog_term::TermDecorator::new().build();
        // // Refers to the destination (output) for the log origin on bug. 
        // let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        // let console_drain = slog_async::Async::new(console_drain).build().fuse();
        // slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))

        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_envlogger::new(console_drain);
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
    }

}
