use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use crate::models::{TodoList, TodoItem};
use crate::errors::{AppError, AppErrorType};


pub async fn get_todos(
    client: &Client
) -> Result<Vec<TodoList>, AppError> {

    let statement = client
        .prepare( 
            "select * from todo_list order by id desc limit 10" 
        )
        .await
        .map_err( AppError::db_error )?;

    let todos = client.query(&statement, &[])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)
}


pub async fn get_items(
    client: &Client, 
    list_id: i32
) -> Result<Vec<TodoItem>, AppError> {
    
    // Get the latest first
    let statement = client
        .prepare(
            "select * from todo_item where list_id = $1 order by id"
        )
        .await
        .map_err(AppError::db_error)?;

    let items = client.query(&statement, &[&list_id])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();
        
    Ok(items)
}



pub async fn create_todo(
    client: &Client, 
    title: String
) -> Result<TodoList, AppError> {

    let statement = client
        .prepare(
            "insert into todo_list (title) values ($1) returning id, title"
        )
        .await
        .map_err( AppError::db_error )?;

    client.query(&statement, &[&title])
        .await
        .map_err(AppError::db_error)?
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(AppError {
            message: Some("Error creating TODO list".to_string()),
            cause: Some("Unkown error".to_string()),
            error_type: AppErrorType::DbError
        })
}


pub async fn check_item(
    client: &Client, 
    list_id: i32,
    item_id: i32
) -> Result<bool, AppError> {

    let statement = client
        .prepare(
            "update todo_item set checked = true where list_id = $1 and id = $2 and checked = false"
        )
        .await
        .map_err( AppError::db_error )?;

    let result = client.execute(&statement, &[&list_id, &item_id])
        .await
        .map_err(AppError::db_error)?;

    match result {
        ref updated if *updated == 1 => Ok(true),
        _ => Ok(false)
    }
}