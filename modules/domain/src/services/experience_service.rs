use async_trait::async_trait;
use std::sync::Arc;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::entities::experience_entity::ExperienceEntity;
use crate::repositories::experience_repository::ExperienceRepository;

#[async_trait]
pub trait ExperienceService: Send + Sync {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<ExperienceEntity>>;
    async fn find_by_company(&self, company: &str) -> DomainResponse<Option<ExperienceEntity>>;
    #[allow(clippy::too_many_arguments)]
    async fn create_experience(
        &self,
        account_id: &str,
        technologies: &'life0 [String],
        position: &str,
        responsibility: &'life1 [String],
        company: &str,
        location: &str,
        start_date: i64,
        end_date: Option<i64>,
        is_current: bool,
    ) -> DomainResponse<ExperienceEntity>;
}

pub struct ExperienceServiceImpl {
    repository: Arc<dyn ExperienceRepository>,
}

impl ExperienceServiceImpl {
    pub fn new(repository: Arc<dyn ExperienceRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ExperienceService for ExperienceServiceImpl {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<ExperienceEntity>> {
        self.repository.find_by_account_id(account_id).await
    }

    async fn find_by_company(&self, company: &str) -> DomainResponse<Option<ExperienceEntity>> {
        self.repository.find_by_company(company).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn create_experience(
        &self,
        account_id: &str,
        technologies: &'life0 [String],
        position: &str,
        responsibility: &'life1 [String],
        company: &str,
        location: &str,
        start_date: i64,
        end_date: Option<i64>,
        is_current: bool,
    ) -> DomainResponse<ExperienceEntity> {
        let experience_entity = ExperienceEntity::new(
            false,
            account_id,
            technologies,
            position,
            responsibility,
            company,
            location,
            start_date,
            end_date,
            is_current,
        )?;

        self.repository.create(&experience_entity).await
    }
}
