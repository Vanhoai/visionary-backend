use crate::entities::provider_entity::ProviderEntity;
use crate::repositories::base_repository::BaseRepository;
use async_trait::async_trait;
use shared::types::DomainResponse;

#[async_trait]
pub trait ProviderRepository: BaseRepository<ProviderEntity> {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<ProviderEntity>>;
}
