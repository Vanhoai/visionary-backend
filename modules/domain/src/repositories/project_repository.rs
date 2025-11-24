use async_trait::async_trait;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::{entities::project_entity::ProjectEntity, repositories::base_repository::BaseRepository};

#[async_trait]
pub trait ProjectRepository: BaseRepository<ProjectEntity> {
    async fn find_by_name(&self, name: &str) -> DomainResponse<Option<ProjectEntity>>;
    async fn update_project_partial(
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
