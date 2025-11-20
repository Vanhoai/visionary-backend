use async_trait::async_trait;
use std::sync::Arc;

// shared modules
use shared::{models::failure::Failure, types::DomainResponse};

// internal modules
use crate::{
    entities::{experience_entity::ExperienceEntity, project_entity::ProjectEntity},
    services::{experience_service::ExperienceService, project_service::ProjectService},
    usecases::global_usecases::{
        AddExperienceParams, AddProjectParams, ManageExperienceUseCase, ManageProjectUseCase, UpdateProjectParams,
    },
};

pub struct GlobalAppService {
    experience_service: Arc<dyn ExperienceService>,
    project_service: Arc<dyn ProjectService>,
}

impl GlobalAppService {
    pub fn new(experience_service: Arc<dyn ExperienceService>, project_service: Arc<dyn ProjectService>) -> Self {
        Self { experience_service, project_service }
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

// region ============================= ManageProjectUseCase =============================
#[async_trait]
impl ManageProjectUseCase for GlobalAppService {
    async fn add_project(&self, params: &AddProjectParams) -> DomainResponse<ProjectEntity> {
        // 1. Check for existing project with same name
        let existing_project = self.project_service.find_project_with_name(&params.name).await?;
        if existing_project.is_some() {
            return Err(Failure::Conflict(format!("Project with name {} already exists", params.name)));
        }

        // 2. Create new project
        let project = self
            .project_service
            .add_project(
                &params.cover,
                &params.name,
                &params.description,
                &params.link,
                &params.github,
                &params.tags,
                &params.markdown,
            )
            .await?;

        Ok(project)
    }

    async fn remove_project_with_id(&self, id: &str) -> DomainResponse<()> {
        self.project_service.remove_project_with_id(id).await
    }

    async fn find_project_with_id(&self, id: &str) -> DomainResponse<Option<ProjectEntity>> {
        self.project_service.find_project_with_id(id).await
    }

    async fn find_projects(&self) -> DomainResponse<Vec<ProjectEntity>> {
        self.project_service.find_projects().await
    }

    async fn update_project(&self, id: &str, params: &UpdateProjectParams) -> DomainResponse<ProjectEntity> {
        todo!()
    }
}
// endregion ============================= ManageProjectUseCase =============================
