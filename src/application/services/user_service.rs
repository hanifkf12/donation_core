use crate::domain::models::user::User;
use crate::infrastructure::database::repositories::user_repository::UserRepository;
use sqlx::{Error, PgPool};
use std::sync::Arc;
use crate::application::services::service::{Service, ReadService, WriteService};
use crate::application::use_cases::users::create_user::CreateUserRequest;
use async_trait::async_trait;
use uuid::Uuid;

pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        UserService { user_repo }
    }
}

#[async_trait]
impl Service for UserService {
    type Error = Error;
}

#[async_trait]
impl ReadService for UserService {
    type ReadModel = User;

    async fn get_all(&self, pool: &PgPool) -> Result<Vec<Self::ReadModel>, Self::Error> {
        self.user_repo.get_users(pool).await
    }

    async fn get_by_id(&self, pool: &PgPool, id: Uuid) -> Result<Option<Self::ReadModel>, Self::Error> {
       self.user_repo.get_user_by_id(pool, id).await
    }
}

#[async_trait]
impl WriteService for UserService {
    type WriteModel = User;
    type CreatePayload = CreateUserRequest;
    type UpdatePayload = CreateUserRequest;

    async fn create(&self, pool: &PgPool, payload: Self::CreatePayload) -> Result<Self::WriteModel, Self::Error> {
        self.user_repo.create_user(pool, payload).await
    }

    async fn update(&self, pool: &PgPool, id: Uuid, payload: Self::UpdatePayload) -> Result<Self::WriteModel, Self::Error> {
        self.user_repo.update_user(pool, id, payload).await
    }

    async fn delete(&self, pool: &PgPool, id: Uuid) -> Result<(), Self::Error> {
        self.user_repo.delete_user(pool, id).await
    }

}
