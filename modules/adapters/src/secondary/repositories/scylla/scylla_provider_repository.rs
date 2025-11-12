use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;

// shared modules
use domain::entities::provider_entity::ProviderEntity;
use domain::repositories::provider_repository::ProviderRepository;
use shared::types::DomainResponse;

// internal modules
use crate::impl_scylla_base_repository;
use crate::secondary::repositories::models::provider_schema::ScyllaProviderSchema;
use crate::secondary::repositories::scylla::scylla_base_repository::ScyllaBaseRepository;

#[allow(dead_code)]
pub struct ScyllaProviderRepository {
    base: ScyllaBaseRepository<ProviderEntity, ScyllaProviderSchema>,
}

impl ScyllaProviderRepository {
    pub fn new(session: Arc<Session>, keyspace: &str) -> Self {
        ScyllaProviderRepository { base: ScyllaBaseRepository::new(session, keyspace) }
    }
}

impl_scylla_base_repository!(ScyllaProviderRepository, ProviderEntity, ScyllaProviderSchema);

#[async_trait]
impl ProviderRepository for ScyllaProviderRepository {
    async fn find_by_account_id(&self, _account_id: &str) -> DomainResponse<Vec<ProviderEntity>> {
        todo!()
    }
}
