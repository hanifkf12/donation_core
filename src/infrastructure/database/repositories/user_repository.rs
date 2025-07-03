use crate::domain::models::user::User;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_users(&self, pg_pool: &PgPool) -> Result<Vec<User>, sqlx::Error>;
}
