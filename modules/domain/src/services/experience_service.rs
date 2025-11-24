use async_trait::async_trait;
use shared::models::failure::Failure;
use std::sync::Arc;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::entities::experience_entity::ExperienceEntity;
use crate::repositories::experience_repository::ExperienceRepository;

#[async_trait]
pub trait ExperienceService: Send + Sync {
    async fn find_by_company(&self, company: &str) -> DomainResponse<Option<ExperienceEntity>>;

    #[allow(clippy::too_many_arguments)]
    async fn create_experience(
        &self,
        technologies: &[String],
        position: &str,
        responsibility: &[String],
        company: &str,
        location: &str,
        start_date: i64,
        end_date: Option<i64>,
        is_current: bool,
    ) -> DomainResponse<ExperienceEntity>;

    async fn find_experiences(&self) -> DomainResponse<Vec<ExperienceEntity>>;
    async fn remove_experience_with_id(&self, id: &str) -> DomainResponse<()>;

    #[allow(clippy::too_many_arguments)]
    async fn update_experience(
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
    async fn find_by_company(&self, company: &str) -> DomainResponse<Option<ExperienceEntity>> {
        self.repository.find_by_company(company).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn create_experience(
        &self,
        technologies: &[String],
        position: &str,
        responsibility: &[String],
        company: &str,
        location: &str,
        start_date: i64,
        end_date: Option<i64>,
        is_current: bool,
    ) -> DomainResponse<ExperienceEntity> {
        let experience_entity = ExperienceEntity::new(
            false,
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

    async fn find_experiences(&self) -> DomainResponse<Vec<ExperienceEntity>> {
        self.repository.finds().await
    }

    async fn remove_experience_with_id(&self, id: &str) -> DomainResponse<()> {
        let row_affected = self.repository.remove(id).await?;
        if row_affected == 0 {
            return Err(Failure::NotFound(format!("Experience with id {} not found", id)));
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn update_experience(
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
    ) -> DomainResponse<ExperienceEntity> {
        if let Some(ref techs) = technologies {
            ExperienceEntity::validate_technologies(techs.to_owned())?;
        }

        if let Some(ref pos) = position {
            ExperienceEntity::validate_position(pos)?;
        }

        if let Some(ref resp) = responsibility {
            ExperienceEntity::validate_responsibility(resp.to_owned())?;
        }

        if let Some(ref comp) = company {
            ExperienceEntity::validate_company(comp)?;
        }

        if let Some(ref loc) = location {
            ExperienceEntity::validate_location(loc)?;
        }

        if start_date.is_some() || end_date.is_some() {
            let start = start_date.unwrap_or(0);
            let end = end_date.unwrap_or(None);
            ExperienceEntity::validate_dates(start, end)?;
        }

        let updated_experience = self
            .repository
            .update_experience_partial(
                id,
                technologies,
                position,
                responsibility,
                company,
                location,
                start_date,
                end_date,
                is_current,
            )
            .await?;

        Ok(updated_experience)
    }
}
