use crate::domain::models::user::User;
use crate::infrastructure::database::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use sqlx::{PgPool, query_as};
use crate::application::use_cases::users::create_user::CreateUserRequest;
use chrono::Utc;
use uuid::Uuid;

pub struct SqlxProductRepositoryImpl;

#[async_trait]
impl UserRepository for SqlxProductRepositoryImpl {
    async fn get_users(&self, pg_pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        query_as!(User, "SELECT * FROM users WHERE deleted_at IS NULL;")
            .fetch_all(pg_pool)
            .await
    }

    async fn create_user(&self, pg_pool: &PgPool, request: CreateUserRequest) -> Result<User, sqlx::Error> {
        let now = Utc::now().naive_utc();
        let user = query_as!(User,
            "INSERT INTO users (id, username, email, password, is_verified, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) 
             RETURNING *;",
            Uuid::new_v4(),
            request.username,
            request.email,
            request.password,
            false,
            now,
            now
        )
        .fetch_one(pg_pool)
        .await?;

        Ok(user)
    }
}
