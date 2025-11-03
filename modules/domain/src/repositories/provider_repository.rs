use async_trait::async_trait;
use crate::entities::provider_entity::ProviderEntity;
use crate::repositories::base_repository::BaseRepository;

#[async_trait]
pub trait ProviderRepository: BaseRepository<ProviderEntity> {}
