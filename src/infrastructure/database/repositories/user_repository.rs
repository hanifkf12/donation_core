use crate::domain::models::user::User;
use async_trait::async_trait;
use sqlx::PgPool;
use crate::application::use_cases::users::create_user::CreateUserRequest;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_users(&self, pg_pool: &PgPool) -> Result<Vec<User>, sqlx::Error>;
    async fn create_user(&self, pg_pool: &PgPool, request: CreateUserRequest) -> Result<User, sqlx::Error>;
}
