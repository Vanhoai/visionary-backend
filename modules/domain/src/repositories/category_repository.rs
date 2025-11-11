use async_trait::async_trait;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::{entities::category_entity::CategoryEntity, repositories::base_repository::BaseRepository};

#[async_trait]
pub trait CategoryRepository: BaseRepository<CategoryEntity> {
    async fn find_by_name(&self, name: &str) -> DomainResponse<Option<CategoryEntity>>;
}
