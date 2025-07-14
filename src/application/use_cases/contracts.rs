use crate::application::services::service::Service;
use sqlx::PgPool;

#[async_trait::async_trait]
pub trait Command<S, Input = (), Output = ()>: Send + Sync 
where
    S: Service,
{
    async fn execute(
        &self,
        service: &S,
        pool: &PgPool,
        input: Input,
    ) -> Result<Output, S::Error>;
}
