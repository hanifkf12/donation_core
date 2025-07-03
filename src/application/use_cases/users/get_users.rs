use crate::application::services::user_service::UserService;
use crate::application::use_cases::contracts::Command;
use crate::domain::models::user::User;
use async_trait::async_trait;
use sqlx::{Error, PgPool};

pub struct GetUsersUseCase;

#[async_trait]
impl Command for GetUsersUseCase {
    type Output = Option<Vec<User>>;

    async fn execute(
        &self,
        user_service: &UserService,
        pool: &PgPool,
    ) -> Result<Self::Output, Error> {
        match user_service.get_users(pool).await {
            Ok(result) => Ok(Some(result)),
            Err(e) => Err(e),
        }
    }
}
