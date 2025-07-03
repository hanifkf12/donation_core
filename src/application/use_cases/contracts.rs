use crate::application::services::user_service::UserService;
use sqlx::{PgPool};

#[async_trait::async_trait]
pub trait Command: Send + Sync {
    type Output;
    async fn execute(
        &self,
        user_service: &UserService,
        pool: &PgPool,
    ) -> Result<Self::Output, sqlx::Error>;
}
