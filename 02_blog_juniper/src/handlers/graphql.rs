use crate::{
    config::HashingService,  
    errors::{AppError},
    models::user::{User, CreateUser},
    repositories::user::UserRepository
};

use uuid::Uuid;
use std::sync::Arc;
use juniper::{RootNode};
use deadpool_postgres::Pool;



#[derive(Clone)]
pub struct Context {
    pub pool: Arc<Pool>,
    pub hashing: Arc<HashingService>
}

impl Context {
    pub fn user_repository(&self) -> UserRepository {
        UserRepository::new(self.pool.clone())
    }
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

    pub async fn user(id: Uuid, context: &Context) -> Result<User, AppError> {
        context.user_repository().get(id).await
    }

    pub async fn users(context: &Context) -> Result<Vec<User>, AppError> {
        context.user_repository().all().await
    }
}

pub struct Mutation {} 


#[juniper::graphql_object(
    Context = Context,
)]
impl Mutation {
    async fn create_user(input: CreateUser, context: &Context) -> Result<User, AppError> {
        context.user_repository().create(input, context.hashing.clone()).await   
    }
}



pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation{})
}

