use serde::{Deserialize, Serialize};
use crate::application::services::user_service::UserService;
use crate::application::services::service::WriteService;
use crate::application::use_cases::contracts::Command;
use crate::domain::models::user::User;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct CreateUserUseCase;

#[async_trait::async_trait]
impl Command<UserService, CreateUserRequest, User> for CreateUserUseCase {
    async fn execute(
        &self,
        service: &UserService,
        pool: &PgPool,
        input: CreateUserRequest,
    ) -> Result<User, sqlx::Error> {
        service.create(pool, input).await
    }
}