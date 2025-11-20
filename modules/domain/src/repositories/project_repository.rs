use async_trait::async_trait;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::{entities::project_entity::ProjectEntity, repositories::base_repository::BaseRepository};

#[async_trait]
pub trait ProjectRepository: BaseRepository<ProjectEntity> {
    async fn find_by_name(&self, name: &str) -> DomainResponse<Option<ProjectEntity>>;
}
