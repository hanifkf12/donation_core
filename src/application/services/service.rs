use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait Service: Send + Sync {
    type Error;
}

#[async_trait]
pub trait ReadService: Service {
    type ReadModel;
    
    async fn get_all(&self, pool: &PgPool) -> Result<Vec<Self::ReadModel>, Self::Error>;
    async fn get_by_id(&self, pool: &PgPool, id: uuid::Uuid) -> Result<Option<Self::ReadModel>, Self::Error>;
}

#[async_trait]
pub trait WriteService: Service {
    type WriteModel;
    type CreatePayload;
    type UpdatePayload;
    
    async fn create(&self, pool: &PgPool, payload: Self::CreatePayload) -> Result<Self::WriteModel, Self::Error>;
    async fn update(&self, pool: &PgPool, id: uuid::Uuid, payload: Self::UpdatePayload) -> Result<Self::WriteModel, Self::Error>;
    async fn delete(&self, pool: &PgPool, id: uuid::Uuid) -> Result<(), Self::Error>;
}