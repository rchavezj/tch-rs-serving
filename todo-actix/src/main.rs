mod config;
mod models;

// use crate::models::Status;
use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::io;

async fn status() -> impl Responder {
    // web::HttpResponse::Ok().json(Status {
    //     status: "UP".to_string(),
    // })
    "{\"Status\": \"HALO 3\"}"
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let _config = config::Config::from_env().unwrap();
    // println!("yolo");
    // let pool = config.pg.create_poo(NoTls).unwrap();

    // println!(config);

    // println!("{}", config.server.host);
    println!("d");
    // println!(
    //     "Starting server at http://{}:{}/",
    //     _config.server.host, _config.server.port
    // );

    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
