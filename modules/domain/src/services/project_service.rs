use async_trait::async_trait;
use std::sync::Arc;

// shared modules
use shared::{models::failure::Failure, types::DomainResponse};

// internal modules
use crate::{entities::project_entity::ProjectEntity, repositories::project_repository::ProjectRepository};

#[async_trait]
pub trait ProjectService: Send + Sync {
    #[allow(clippy::too_many_arguments)]
    async fn add_project(
        &self,
        cover: &str,
        name: &str,
        description: &str,
        link: &str,
        github: &str,
        tags: &[String],
        markdown: &str,
    ) -> DomainResponse<ProjectEntity>;

    async fn find_project_with_name(&self, name: &str) -> DomainResponse<Option<ProjectEntity>>;
    async fn remove_project_with_id(&self, id: &str) -> DomainResponse<()>;
    async fn find_projects(&self) -> DomainResponse<Vec<ProjectEntity>>;
    async fn find_project_with_id(&self, id: &str) -> DomainResponse<Option<ProjectEntity>>;

    #[allow(clippy::too_many_arguments)]
    async fn update_project(
        &self,
        id: &str,
        cover: Option<String>,
        name: Option<String>,
        description: Option<String>,
        link: Option<String>,
        github: Option<String>,
        tags: Option<Vec<String>>,
        markdown: Option<String>,
    ) -> DomainResponse<ProjectEntity>;
}

pub struct ProjectServiceImpl {
    repository: Arc<dyn ProjectRepository>,
}

impl ProjectServiceImpl {
    pub fn new(repository: Arc<dyn ProjectRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ProjectService for ProjectServiceImpl {
    #[allow(clippy::too_many_arguments)]
    async fn add_project(
        &self,
        cover: &str,
        name: &str,
        description: &str,
        link: &str,
        github: &str,
        tags: &[String],
        markdown: &str,
    ) -> DomainResponse<ProjectEntity> {
        ProjectEntity::validate_name(name)?;
        ProjectEntity::validate_description(description)?;

        let project = ProjectEntity::new(true, cover, name, description, link, github, tags, markdown);
        self.repository.create(&project).await
    }

    async fn find_project_with_name(&self, name: &str) -> DomainResponse<Option<ProjectEntity>> {
        self.repository.find_by_name(name).await
    }

    async fn remove_project_with_id(&self, id: &str) -> DomainResponse<()> {
        let match_count = self.repository.remove(id).await?;
        if match_count == 0 {
            return Err(Failure::NotFound(format!("Project with id {} not found", id)));
        }

        Ok(())
    }

    async fn find_projects(&self) -> DomainResponse<Vec<ProjectEntity>> {
        self.repository.finds().await
    }

    async fn find_project_with_id(&self, id: &str) -> DomainResponse<Option<ProjectEntity>> {
        self.repository.find(id).await
    }

    #[allow(clippy::too_many_arguments)]
    async fn update_project(
        &self,
        id: &str,
        cover: Option<String>,
        name: Option<String>,
        description: Option<String>,
        link: Option<String>,
        github: Option<String>,
        tags: Option<Vec<String>>,
        markdown: Option<String>,
    ) -> DomainResponse<ProjectEntity> {
        if let Some(ref name) = name {
            ProjectEntity::validate_name(name)?;
        }
        if let Some(ref description) = description {
            ProjectEntity::validate_description(description)?;
        }

        let updated =
            self.repository.update_project_partial(id, cover, name, description, link, github, tags, markdown).await?;

        Ok(updated)
    }
}
