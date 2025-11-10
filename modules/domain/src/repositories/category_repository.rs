use crate::{entities::category_entity::CategoryEntity, repositories::base_repository::BaseRepository};
use async_trait::async_trait;
use shared::types::DomainResponse;

#[async_trait]
pub trait CategoryRepository: BaseRepository<CategoryEntity> {
    async fn find_by_name(&self, name: &str) -> DomainResponse<Option<CategoryEntity>>;
}
