use crate::application::services::service::WriteService;
use crate::application::services::user_service::UserService;
use crate::application::use_cases::contracts::Command;
use crate::application::use_cases::users::create_user::CreateUserRequest;
use crate::domain::models::user::User;
use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct UpdateUserUseCase;

#[async_trait]
impl Command<UserService, (Uuid, CreateUserRequest), User> for UpdateUserUseCase {
    async fn execute(
        &self,
        service: &UserService,
        pool: &PgPool,
        input: (Uuid, CreateUserRequest),
    ) -> Result<User, sqlx::Error> {
        let (id, request) = input;
        service.update(pool, id, request).await
    }
}
