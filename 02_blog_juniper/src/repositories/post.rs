use crate::{
    config::HashingService,
    errors::{AppError, AppErrorType},
    models::post::{Post, CreatePost}
};
use std::{
    sync::Arc,
    collections::HashMap
};
use uuid::Uuid;
use slog_scope::error;
use deadpool_postgres::{Client, Pool};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::error::{Error, SqlState};


pub struct PostRepository {
    pool: Arc<Pool>
}


pub struct PostBatcher {
    pool: Arc<Pool>
}

impl PostBatcher {
    pub async fn get_posts_by_user_ids(
        &self, 
        hashmap: HashMap<Uuid, Vec<Post>>, 
        ids: Vec<Uuid>
    ) -> Result<(), AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "get_posts_by_user_ids");
                err
            })?;

        let statement = client.prepare("select * from posts where author_id = ANY($1)").await?;

        client
            .query(&statement, &[&ids])
            .await
            .map_err(|err| {
                error!("Error getting posts. {}", err; "query" => "get_posts_by_user_ids");
                err
            })?
            .iter()
            .map(|row| Post::from_row_ref(row))
            .collect::<Result<Vec<Post>, _>>()
            .map_err(|err| {
                error!("Error getting parsing posts. {}", err; "query" => "get_posts_by_user_ids");
                err
            })?;

        Ok(())
    }
}



impl PostRepository {
    pub fn new(pool: Arc<Pool>) -> PostRepository{
        PostRepository{ pool }
    }

    pub async fn get(&self, id: Uuid) -> Result<Post, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "get");
                err
            })?;

        let statement = client.prepare("select * from post where id = $1").await?;

        client
            .query(&statement, &[&id])
            .await
            .map_err(|err| {
                error!("Error getting posts. {}", err; "query" => "get");
                err
            })?
            .iter()
            .map(|row| Post::from_row_ref(row))
            .collect::<Result<Vec<Post>, _>>()?
            .pop()
            .ok_or( AppError {
                cause: None,
                message: Some(format!("Post with id {} not found", id)),
                error_type: AppErrorType::NotFoundError
            })
    }


    pub async fn get_for_user(&self, user_id: Uuid) -> Result<Vec<Post>, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "posts");
                err
            })?;

        let statement = client.prepare("select * from posts where author_id = $1").await?;

        let posts = client
            .query(&statement, &[&user_id])
            .await
            .map_err(|err| {
                error!("Error getting posts. {}", err; "query" => "posts");
                err
            })?
            .iter()
            .map(|row| Post::from_row_ref(row))
            .collect::<Result<Vec<Post>, _>>()
            .map_err(|err| {
                error!("Error getting parsing posts. {}", err; "query" => "posts");
                err
            })?;

        Ok(posts)
    }


    pub async fn all(&self) -> Result<Vec<Post>, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "posts");
                err
            })?;

        let statement = client.prepare("select * from posts").await?;

        let posts = client
            .query(&statement, &[])
            .await
            .map_err(|err| {
                error!("Error getting posts. {}", err; "query" => "posts");
                err
            })?
            .iter()
            .map(|row| Post::from_row_ref(row))
            .collect::<Result<Vec<Post>, _>>()
            .map_err(|err| {
                error!("Error getting parsing posts. {}", err; "query" => "posts");
                err
            })?;

        Ok(posts)
    }

    pub async fn create(&self, input: CreatePost, hashing: Arc<HashingService>) -> Result<Post, AppError> {
        let client: Client = self.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "create_post");
                err
            })?;

        let statement = client
            .prepare("insert into posts (author_id, slug, title, description, body) values($1, $2, $3, $4, $5) returning *")
            .await?;

        let slug = match input.slug {
            Some(s) => s,
            None => Uuid::new_v4().to_string()
        };

        let author_id = input.author_id.clone();

        let post = client
            .query(&statement, &[
                &input.author_id,
                &slug,
                &input.title,
                &input.description,
                &input.body
            ])
            .await
            .map_err(|err: Error| {
                match err.code(){
                    Some(code) => match code{
                        c if c == &SqlState::UNIQUE_VIOLATION => AppError {
                            cause: Some(err.to_string()),
                            message: Some(format!("Slug {} already exists.", slug)),
                            error_type: AppErrorType::InvalidField
                        },
                        c if c == &SqlState::FOREIGN_KEY_VIOLATION => AppError {
                            cause: Some(err.to_string()),
                            message: Some(format!("Author with id {} doesn't exists.", author_id)),
                            error_type: AppErrorType::InvalidField
                        },
                        _ => AppError::from(err)
                    },
                    _ => AppError::from(err)
                }
            })?
            .iter()
            .map(|row| Post::from_row_ref(row))
            .collect::<Result<Vec<Post>, _>>()?
            .pop()
            .ok_or(AppError {
                message: Some("Error creating Post.".to_string()),
                cause: Some("Unknown error.".to_string()),
                error_type: AppErrorType::DbError
            })?;

        Ok(post)
    }
}