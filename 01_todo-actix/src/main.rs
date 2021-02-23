mod db;
mod errors;
mod config;
mod models;
mod handlers;

use std::io;
use slog::info;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use actix_web::{web, App, HttpServer};

use crate::handlers::*;
use crate::config::Config;
use crate::models::AppState;


#[actix_rt::main]
async fn main() -> io::Result<()> {

    dotenv().ok();

    let config = Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();
    
    let log = Config::configure_log();

    info!(
        log,
        "Starting server at http://{}:{}/",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                pool: pool.clone(),
                log: log.clone()
            })
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/todos/{list_id}/items{_:/?}", web::get().to(get_items))
            .route("/todos/{list_id}/items/{item_id}{_:/?}", web::put().to(check_item))
    })        
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

#[cfg(test)]
#[cfg(feature = "integration_tests")]
mod integration_tests;

