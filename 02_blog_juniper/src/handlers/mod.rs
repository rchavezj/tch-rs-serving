mod graphql;

use crate::config::HashingService;

use std::sync::Arc;
use deadpool_postgres::Pool;
use actix_web::{web, HttpResponse};
use graphql::{create_schema, Schema, Context};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};


async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn app_config(config: &mut web::ServiceConfig) {
    let schema = create_schema();
    config
        .data(schema)
        .service(web::resource("/graphql").route(web::post().to(graphql)))
        .service(web::resource("/graphiql").route(web::get().to(graphiql)))
        .service(web::resource("/").route(web::get().to(health)));
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    data: web::Json<GraphQLRequest>,
    schema: web::Data<Schema>,
    pool: web::Data<Pool>,
    hashing_service: web::Data<HashingService>
) -> HttpResponse {
    let pool: Arc<Pool> = pool.into_inner();
    let hashing = hashing_service.into_inner();
    let context = Context { pool, hashing };
    let res = data.execute(&schema, &context).await;
    HttpResponse::Ok().json(res)
}