use crate::domain::models::user::User;
use crate::infrastructure::database::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use sqlx::{PgPool, query_as};
use crate::application::use_cases::users::create_user::CreateUserRequest;
use chrono::Utc;
use uuid::Uuid;

pub struct SqlxUserRepositoryImpl;

#[async_trait]
impl UserRepository for SqlxUserRepositoryImpl {
    async fn get_users(&self, pg_pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        query_as!(User, "SELECT * FROM users WHERE deleted_at IS NULL;")
            .fetch_all(pg_pool)
            .await
    }

    async fn get_user_by_id(&self, pg_pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        query_as!(User, 
            "SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL;",
            id
        )
        .fetch_optional(pg_pool)
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

    async fn update_user(&self, pg_pool: &PgPool, id: Uuid, request: CreateUserRequest) -> Result<User, sqlx::Error> {
        let now = Utc::now().naive_utc();
        let user = query_as!(User,
            "UPDATE users 
             SET username = $1, email = $2, password = $3, updated_at = $4
             WHERE id = $5 AND deleted_at IS NULL
             RETURNING *;",
            request.username,
            request.email,
            request.password,
            now,
            id
        )
        .fetch_one(pg_pool)
        .await?;

        Ok(user)
    }

    async fn delete_user(&self, pg_pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
        let now = Utc::now().naive_utc();
        query_as!(User,
            "UPDATE users 
             SET deleted_at = $1
             WHERE id = $2 AND deleted_at IS NULL
             RETURNING *;",
            now,
            id
        )
        .fetch_one(pg_pool)
        .await?;

        Ok(())
    }
}
