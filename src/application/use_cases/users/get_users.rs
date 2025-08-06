use crate::application::services::service::ReadService;
use crate::application::services::user_service::UserService;
use crate::application::use_cases::contracts::Command;
use crate::domain::models::user::User;
use async_trait::async_trait;
use log::error;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct GetUsersUseCase;
pub struct GetUserByIdUseCase;

#[async_trait]
impl Command<UserService, (), Option<Vec<User>>> for GetUsersUseCase {
    async fn execute(
        &self,
        service: &UserService,
        pool: &PgPool,
        _input: (),
    ) -> Result<Option<Vec<User>>, Error> {
        match service.get_all(pool).await {
            Ok(users) => Ok(Some(users)),
            Err(e) => {
                error!("GetUsersUseCase failed: {}", e);
                Err(e)
            }
        }
    }
}

#[async_trait]
impl Command<UserService, Uuid, Option<User>> for GetUserByIdUseCase {
    async fn execute(
        &self,
        service: &UserService,
        pool: &PgPool,
        id: Uuid,
    ) -> Result<Option<User>, Error> {
        match service.get_by_id(pool, id).await {
            Ok(user) => Ok(user),
            Err(e) => {
                error!("GetUserByIdUseCase failed: {}, id: {}", e, id);
                Err(e)
            }
        }
    }
}
