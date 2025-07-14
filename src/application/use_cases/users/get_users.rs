use crate::application::services::user_service::UserService;
use crate::application::services::service::ReadService;
use crate::application::use_cases::contracts::Command;
use crate::domain::models::user::User;
use async_trait::async_trait;
use sqlx::PgPool;

pub struct GetUsersUseCase;

#[async_trait]
impl Command<UserService, (), Option<Vec<User>>> for GetUsersUseCase {
    async fn execute(
        &self,
        service: &UserService,
        pool: &PgPool,
        _input: (),
    ) -> Result<Option<Vec<User>>, sqlx::Error> {
        match service.get_all(pool).await {
            Ok(result) => Ok(Some(result)),
            Err(e) => Err(e),
        }
    }
}
