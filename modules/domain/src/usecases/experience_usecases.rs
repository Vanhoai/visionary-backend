use async_trait::async_trait;
use serde::Deserialize;

// shared modules
use shared::models::paginate::Paginate;
use shared::types::DomainResponse;

// internal modules
use crate::entities::experience_entity::ExperienceEntity;

// region ============================= MANAGE EXPERIENCE USE CASE =============================
#[derive(Debug, Clone, Deserialize)]
pub struct FindExperiencesQuery {
    #[serde(flatten)]
    pub paginate: Paginate,
}

pub struct UpdateExperienceParams {
    pub position: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub technologies: Option<Vec<String>>,
    pub responsibility: Option<Vec<String>>,
    pub start_date: u64,
    pub end_date: Option<u64>,
    pub is_current: Option<bool>,
}

#[async_trait]
pub trait ManageExperienceUseCase: Send + Sync {
    async fn find_experiences(&self, query: &FindExperiencesQuery)
    -> DomainResponse<(Paginate, Vec<ExperienceEntity>)>;

    async fn update_experience(
        &self,
        experience_id: &str,
        params: &UpdateExperienceParams,
    ) -> DomainResponse<ExperienceEntity>;
}
// endregion ============================= MANAGE EXPERIENCE USE CASE =============================
