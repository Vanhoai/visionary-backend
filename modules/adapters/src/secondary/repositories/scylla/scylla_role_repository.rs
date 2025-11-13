use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;

// shared modules
use domain::entities::role_entity::RoleEntity;
use domain::repositories::role_repository::RoleRepository;
use shared::types::DomainResponse;

// internal modules
use crate::impl_scylla_base_repository;
use crate::secondary::repositories::models::role_schema::ScyllaRoleSchema;
use crate::secondary::repositories::scylla::scylla_base_repository::ScyllaBaseRepository;

#[allow(dead_code)]
pub struct ScyllaRoleRepository {
    base: ScyllaBaseRepository<RoleEntity, ScyllaRoleSchema>,
}

impl ScyllaRoleRepository {
    pub fn new(session: Arc<Session>, keyspace: &str, table_name: &str) -> Self {
        ScyllaRoleRepository { base: ScyllaBaseRepository::new(session, keyspace, table_name) }
    }
}

impl_scylla_base_repository!(ScyllaRoleRepository, RoleEntity, ScyllaRoleSchema);

#[async_trait]
impl RoleRepository for ScyllaRoleRepository {
    async fn find_by_account_id(&self, _account_id: &str) -> DomainResponse<Option<RoleEntity>> {
        todo!()
    }

    async fn find_and_update_role_by_account_id(
        &self,
        _account_id: &str,
        _role_name: &str,
    ) -> DomainResponse<RoleEntity> {
        todo!()
    }
}
