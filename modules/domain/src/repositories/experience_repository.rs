use crate::entities::experience_entity::ExperienceEntity;
use crate::repositories::base_repository::BaseRepository;
use async_trait::async_trait;
use shared::types::DomainResponse;

#[async_trait]
pub trait ExperienceRepository: BaseRepository<ExperienceEntity> {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<ExperienceEntity>>;
    async fn find_by_company(&self, company: &str) -> DomainResponse<Option<ExperienceEntity>>;
}
