use async_trait::async_trait;
use shared::types::DomainResponse;

#[async_trait]
pub trait BaseRepository<T>: Send + Sync {
    async fn create(&self, entity: T) -> DomainResponse<T>;
    async fn update(&self, id: &str, entity: T) -> DomainResponse<T>;
    async fn delete(&self, id: &str) -> DomainResponse<T>;
    async fn remove(&self, id: &str) -> DomainResponse<T>;
    async fn find_by_id(&self, id: &str) -> DomainResponse<Option<T>>;
}
