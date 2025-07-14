use crate::application::use_cases::users::create_user::CreateUserRequest;
use crate::domain::models::user::User;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn get_users(&self, pg_pool: &PgPool) -> Result<Vec<User>, sqlx::Error>;
    async fn get_user_by_id(&self, pg_pool: &PgPool, id: Uuid)
    -> Result<Option<User>, sqlx::Error>;
    async fn create_user(
        &self,
        pg_pool: &PgPool,
        request: CreateUserRequest,
    ) -> Result<User, sqlx::Error>;
    async fn update_user(
        &self,
        pg_pool: &PgPool,
        id: Uuid,
        request: CreateUserRequest,
    ) -> Result<User, sqlx::Error>;
    async fn delete_user(&self, pg_pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error>;
}
