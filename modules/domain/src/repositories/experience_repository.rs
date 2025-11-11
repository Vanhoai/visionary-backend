use async_trait::async_trait;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::entities::experience_entity::ExperienceEntity;
use crate::repositories::base_repository::BaseRepository;

#[async_trait]
pub trait ExperienceRepository: BaseRepository<ExperienceEntity> {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<ExperienceEntity>>;
    async fn find_by_company(&self, company: &str) -> DomainResponse<Option<ExperienceEntity>>;
}
