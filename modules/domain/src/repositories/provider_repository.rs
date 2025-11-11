use async_trait::async_trait;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::entities::provider_entity::ProviderEntity;
use crate::repositories::base_repository::BaseRepository;

#[async_trait]
pub trait ProviderRepository: BaseRepository<ProviderEntity> {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<ProviderEntity>>;
}
