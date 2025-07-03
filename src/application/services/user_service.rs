use crate::domain::models::user::User;
use crate::infrastructure::database::repositories::user_repository::UserRepository;
use sqlx::PgPool;
use std::sync::Arc;
pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
}
impl UserService {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        UserService { user_repo }
    }
    pub async fn get_users(&self, pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        self.user_repo.get_users(pool).await
    }
}
