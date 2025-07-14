use crate::application::services::service::WriteService;
use crate::application::use_cases::contracts::Command;
use crate::{application::services::user_service::UserService, domain::models::user::User};
use async_trait::async_trait;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct DeleteUserUseCase;

#[async_trait]
impl Command<UserService, Uuid, ()> for DeleteUserUseCase {
    async fn execute(&self, service: &UserService, pool: &PgPool, id: Uuid) -> Result<(), Error> {
        service.delete(pool, id).await
    }
}
