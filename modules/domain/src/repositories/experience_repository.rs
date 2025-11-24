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
    async fn update_experience_partial(
        &self,
        id: &str,
        technologies: Option<Vec<String>>,
        position: Option<String>,
        responsibility: Option<Vec<String>>,
        company: Option<String>,
        location: Option<String>,
        start_date: Option<i64>,
        end_date: Option<Option<i64>>,
        is_current: Option<bool>,
    ) -> DomainResponse<ExperienceEntity>;
}
