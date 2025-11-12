use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;

// shared modules
use domain::entities::category_entity::CategoryEntity;
use domain::repositories::category_repository::CategoryRepository;
use shared::types::DomainResponse;

// internal modules
use crate::impl_scylla_base_repository;
use crate::secondary::repositories::models::category_schema::ScyllaCategorySchema;
use crate::secondary::repositories::scylla::scylla_base_repository::ScyllaBaseRepository;

#[allow(dead_code)]
pub struct ScyllaCategoryRepository {
    base: ScyllaBaseRepository<CategoryEntity, ScyllaCategorySchema>,
}

impl ScyllaCategoryRepository {
    pub fn new(session: Arc<Session>, keyspace: &str) -> Self {
        ScyllaCategoryRepository { base: ScyllaBaseRepository::new(session, keyspace) }
    }
}

impl_scylla_base_repository!(ScyllaCategoryRepository, CategoryEntity, ScyllaCategorySchema);

#[async_trait]
impl CategoryRepository for ScyllaCategoryRepository {
    async fn find_by_name(&self, _name: &str) -> DomainResponse<Option<CategoryEntity>> {
        todo!()
    }
}
