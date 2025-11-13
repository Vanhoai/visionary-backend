use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;

// shared modules
use domain::entities::session_entity::SessionEntity;
use domain::repositories::session_repository::SessionRepository;
use shared::types::DomainResponse;

// internal modules
use crate::impl_scylla_base_repository;
use crate::secondary::repositories::models::session_schema::ScyllaSessionSchema;
use crate::secondary::repositories::scylla::scylla_base_repository::ScyllaBaseRepository;

#[allow(dead_code)]
pub struct ScyllaSessionRepository {
    base: ScyllaBaseRepository<SessionEntity, ScyllaSessionSchema>,
}

impl ScyllaSessionRepository {
    pub fn new(session: Arc<Session>, keyspace: &str, table_name: &str) -> Self {
        ScyllaSessionRepository { base: ScyllaBaseRepository::new(session, keyspace, table_name) }
    }
}

impl_scylla_base_repository!(ScyllaSessionRepository, SessionEntity, ScyllaSessionSchema);

#[async_trait]
impl SessionRepository for ScyllaSessionRepository {
    async fn remove_by_account_id(&self, _account_id: &str) -> DomainResponse<()> {
        todo!()
    }

    async fn find_by_jit(&self, _jit: &str) -> DomainResponse<Option<SessionEntity>> {
        todo!()
    }
}
