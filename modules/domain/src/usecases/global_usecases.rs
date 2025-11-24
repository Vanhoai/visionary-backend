use async_trait::async_trait;
use serde::Deserialize;
use validator::Validate;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::entities::{experience_entity::ExperienceEntity, project_entity::ProjectEntity};

// region ============================= ManageExperienceUseCase =============================
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

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateExperienceParams {
    pub technologies: Option<Vec<String>>,
    pub position: Option<String>,
    pub responsibility: Option<Vec<String>>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub start_date: Option<i64>,
    pub end_date: Option<Option<i64>>,
    pub is_current: Option<bool>,
}

#[async_trait]
pub trait ManageExperienceUseCase: Send + Sync {
    async fn add_experience(&self, params: &AddExperienceParams) -> DomainResponse<ExperienceEntity>;
    async fn find_experiences(&self) -> DomainResponse<Vec<ExperienceEntity>>;
    async fn remove_experience_with_id(&self, id: &str) -> DomainResponse<()>;
    async fn update_experience(&self, id: &str, params: &UpdateExperienceParams) -> DomainResponse<ExperienceEntity>;
}
// endregion ============================= ManageExperienceUseCase =============================

// region ============================= ManageProjectUseCase =============================
#[derive(Debug, Deserialize, Validate)]
pub struct AddProjectParams {
    #[validate(url)]
    pub cover: String,

    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,

    #[validate(length(min = 1, message = "Description must not be empty"))]
    pub description: String,

    #[validate(url)]
    pub link: String,

    #[validate(url)]
    pub github: String,

    #[validate(length(min = 1, message = "At least one tag must be provided"))]
    pub tags: Vec<String>,

    #[validate(length(min = 1, message = "Markdown content must not be empty"))]
    pub markdown: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectParams {
    pub cover: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub github: Option<String>,
    pub tags: Option<Vec<String>>,
    pub markdown: Option<String>,
}

#[async_trait]
pub trait ManageProjectUseCase: Send + Sync {
    async fn add_project(&self, params: &AddProjectParams) -> DomainResponse<ProjectEntity>;
    async fn remove_project_with_id(&self, id: &str) -> DomainResponse<()>;
    async fn find_project_with_id(&self, id: &str) -> DomainResponse<Option<ProjectEntity>>;
    async fn find_projects(&self) -> DomainResponse<Vec<ProjectEntity>>;
    async fn update_project(&self, id: &str, params: &UpdateProjectParams) -> DomainResponse<ProjectEntity>;
}
// endregion ============================= ManageProjectUseCase =============================
