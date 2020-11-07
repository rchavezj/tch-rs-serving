
use deadpool_postgres::{Pool, Client};
use actix_web::{web, Responder, HttpResponse};

use crate::db;
use crate::errors::{AppError};
use crate::models::{Status, CreateTodoList, ResultResponse};




pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "UP".to_string(),
    })
}


pub async fn get_todos(
    db_pool: web::Data<Pool>
) -> Result<impl Responder, AppError> {
    
    let client: Client = 
        db_pool.get().await
        .map_err( AppError::db_error )?;

    let result = db::get_todos(&client).await;

    result.map(|todos| HttpResponse::Ok().json(todos))

}


pub async fn get_items(
    db_pool: web::Data<Pool>, 
    path: web::Path<(i32,)>
) -> Result<impl Responder, AppError>{

    let client: Client = db_pool
        .get()
        .await
        .map_err( AppError::db_error )?;

    let result = db::get_items(&client, path.0).await;

    result.map(|items| HttpResponse::Ok().json(items))
}


pub async fn create_todo(
    db_pool: web::Data<Pool>, 
    json: web::Json<CreateTodoList>
) -> Result<impl Responder, AppError> {
    
    let client: Client = 
        db_pool.get().await
        .map_err( AppError::db_error )?;

    let result = db::create_todo(&client, json.title.clone()).await;

    result.map(|todo| HttpResponse::Ok().json(todo))
}


pub async fn check_item(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32, i32)>
) -> Result<impl Responder, AppError> {

    let client: Client = 
        db_pool.get().await
        .map_err( AppError::db_error )?;

    //path.0 --> list id, path.1 --> item id
    let result = db::check_item(&client, path.0, path.1).await;
    
    result.map(|updated| HttpResponse::Ok().json(ResultResponse{success: updated}))
}
