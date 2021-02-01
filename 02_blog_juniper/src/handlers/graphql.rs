use deadpool_postgres::Pool;
use juniper::{EmptyMutation, FieldError, RootNode};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::sync::Arc;
use crate::models::user::{User, CreateUser};
use slog_scope::error;

#[derive(Clone)]
pub struct Context {
    pub pool: Arc<Pool>,
}

/// Context Marker
impl juniper::Context for Context {}


pub struct Query {}

#[juniper::graphql_object(
    Context = Context,
)]
impl Query {
    pub async fn api_version() -> &str {
        "1.0"
    }

    pub async fn users(context: &Context) -> Result<Vec<User>, FieldError> {
        let client: Client = context.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "users");
                err
            })?;

        let statement = client.prepare("select * from users").await?;

        let users = client
            .query(&statement, &[])
            .await
            .map_err(|err| {
                error!("Error getting users. {}", err; "query" => "users");
                err
            })?
            .iter()
            .map(|row| User::from_row_ref(row))
            .collect::<Result<Vec<User>, _>>()
            .map_err(|err| {
                error!("Error getting parsing users. {}", err; "query" => "users");
                err
            })?;

        Ok(users)
    }
}

pub struct Mutation {} 


#[juniper::graphql_object]
impl Mutation {
    async fn create_user(input: CreateUser, context: &Context) -> Result<User, FieldError> {
        let client: Client = context.pool
            .get()
            .await
            .map_err(|err| {
                error!("Error getting client {}", err; "query" => "create_user");
                err
            })?;
            
        let state = client
            .prepare(query: "insert into users (username, email, password, bio, image) values($1, $2, $3, $4, $5) returning *").await?;
            
        
    }
}



pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::<Context>::new())
}
