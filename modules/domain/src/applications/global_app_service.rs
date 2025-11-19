use crate::entities::experience_entity::ExperienceEntity;
use crate::services::experience_service::ExperienceService;
use crate::usecases::global_usecases::{AddExperienceParams, ManageExperienceUseCase};
use async_trait::async_trait;
use shared::models::failure::Failure;
use shared::types::DomainResponse;
use std::sync::Arc;

pub struct GlobalAppService {
    experience_service: Arc<dyn ExperienceService>,
}

impl GlobalAppService {
    pub fn new(experience_service: Arc<dyn ExperienceService>) -> Self {
        Self { experience_service }
    }
}

// region ============================== ManageExperienceUseCase ==============================
#[async_trait]
impl ManageExperienceUseCase for GlobalAppService {
    async fn add_experience(&self, params: &AddExperienceParams) -> DomainResponse<ExperienceEntity> {
        // 2. Check for existing experience with same company
        let existing_experience = self.experience_service.find_by_company(&params.company).await?;
        if existing_experience.is_some() {
            return Err(Failure::Conflict(format!("Experience with company {} already exists", params.company)));
        }

        let experience = self
            .experience_service
            .create_experience(
                &params.technologies,
                &params.position,
                &params.responsibility,
                &params.company,
                &params.location,
                params.start_date,
                params.end_date,
                params.is_current,
            )
            .await?;

        Ok(experience)
    }

    async fn find_experiences(&self) -> DomainResponse<Vec<ExperienceEntity>> {
        self.experience_service.find_experiences().await
    }
}
// endregion ============================== ManageExperienceUseCase ==============================
