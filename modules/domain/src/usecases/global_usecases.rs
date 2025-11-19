use async_trait::async_trait;
use serde::Deserialize;
use validator::Validate;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::entities::experience_entity::ExperienceEntity;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddExperienceParams {
    #[validate(length(min = 1, message = "At least one technology must be provided"))]
    pub technologies: Vec<String>,

    #[validate(length(min = 1, message = "Position must not be empty"))]
    pub position: String,

    #[validate(length(min = 1, message = "At least one responsibility must be provided"))]
    pub responsibility: Vec<String>,

    #[validate(length(min = 1, message = "Company must not be empty"))]
    pub company: String,

    #[validate(length(min = 1, message = "Location must not be empty"))]
    pub location: String,

    pub start_date: i64,
    pub end_date: Option<i64>,
    pub is_current: bool,
}

#[async_trait]
pub trait ManageExperienceUseCase: Send + Sync {
    async fn add_experience(&self, params: &AddExperienceParams) -> DomainResponse<ExperienceEntity>;
    async fn find_experiences(&self) -> DomainResponse<Vec<ExperienceEntity>>;
}
