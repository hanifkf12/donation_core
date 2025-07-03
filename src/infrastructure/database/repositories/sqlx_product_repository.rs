use crate::domain::models::user::User;
use crate::infrastructure::database::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use sqlx::{PgPool, query_as};

pub struct SqlxProductRepositoryImpl;

#[async_trait]
impl UserRepository for SqlxProductRepositoryImpl {
    async fn get_users(&self, pg_pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        query_as!(User, "SELECT * FROM users WHERE deleted_at IS NULL;")
            .fetch_all(pg_pool)
            .await
    }
}
