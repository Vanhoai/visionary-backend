use async_trait::async_trait;
use scylla::client::session::Session;
use shared::models::failure::Failure;
use std::sync::Arc;

// shared modules
use domain::entities::category_entity::CategoryEntity;
use domain::repositories::category_repository::CategoryRepository;
use shared::types::DomainResponse;

// internal modules
use crate::impl_scylla_base_repository;
use crate::secondary::repositories::models::category_schema::ScyllaCategorySchema;
use crate::secondary::repositories::scylla::scylla_base_repository::{EntitySchema, ScyllaBaseRepository};

#[allow(dead_code)]
pub struct ScyllaCategoryRepository {
    base: ScyllaBaseRepository<CategoryEntity, ScyllaCategorySchema>,
}

impl ScyllaCategoryRepository {
    pub fn new(session: Arc<Session>, keyspace: &str, table_name: &str) -> Self {
        ScyllaCategoryRepository { base: ScyllaBaseRepository::new(session, keyspace, table_name) }
    }
}

impl_scylla_base_repository!(ScyllaCategoryRepository, CategoryEntity, ScyllaCategorySchema);

#[async_trait]
impl CategoryRepository for ScyllaCategoryRepository {
    async fn find_by_name(&self, name: &str) -> DomainResponse<Option<CategoryEntity>> {
        let query = format!(
            "SELECT {} FROM {}.{} WHERE name = ? ALLOW FILTERING;",
            ScyllaCategorySchema::columns(),
            self.base.keyspace,
            self.base.table_name
        );

        let prepared = self.base.prepare_statement(&query).await?;
        let rows = self
            .base
            .session
            .execute_unpaged(&prepared, vec![name])
            .await
            .map_err(|e| Failure::DatabaseError(format!("Failed to execute query: {}", e)))?
            .into_rows_result();

        println!("find_by_name rows: {:?}", rows);
        Ok(None)
    }
}
