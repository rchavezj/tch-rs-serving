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

    let pool = config.configure_pool();
    
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
mod integration_tests {
    
    use dotenv::dotenv;
    use actix_web::{App, web, test};
    use lazy_static::lazy_static;

    use crate::handlers::*;
    use crate::config::Config;
    use crate::models::AppState;

    lazy_static! {
        static ref APP_STATE: AppState = {
            dotenv().ok();

            let config = Config::from_env().unwrap();

            let pool = config.configure_pool();
    
            let log = Config::configure_log();

            AppState {
                pool: pool.clone(),
                log: log.clone()
            }
        };
    }

    #[actix_rt::test]
    async fn test_get_todos () {

        let app = App::new()
            .data(APP_STATE.clone())
            .route("/todos{_:/?}", web::get().to(get_todos));
        
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get()
            .uri("/todos")
            .to_request();

        let res = test::call_service(&mut app, req).await;

        assert_eq!(res.status(), 200, "GET /todos should return status 200");
    }

    #[actix_rt::test]
    async fn test_create_todos() {
        
    }
}