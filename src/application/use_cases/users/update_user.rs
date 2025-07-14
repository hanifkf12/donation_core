use crate::application::services::user_service::UserService;
use crate::application::use_cases::contracts::Command;
use crate::domain::models::user::User;
use sqlx::{Error, PgPool};
use async_trait::async_trait;
use crate::application::services::service::WriteService;
use uuid::Uuid;
use crate::application::use_cases::users::create_user::CreateUserRequest;

pub struct UpdateUserUseCase;

#[async_trait]
impl Command<UserService, (Uuid, CreateUserRequest), User> for UpdateUserUseCase {
    async fn execute(&self, service: &UserService, pool: &PgPool, input: (Uuid, CreateUserRequest)) -> Result<User, sqlx::Error> {
        let (id, request) = input;
        service.update(pool, id, request).await
    }
}