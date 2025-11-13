use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;

// shared modules
use domain::entities::account_entity::AccountEntity;
use domain::repositories::account_repository::AccountRepository;
use shared::models::failure::Failure;
use shared::types::DomainResponse;

// internal modules
use crate::impl_scylla_base_repository;
use crate::secondary::repositories::models::account_schema::ScyllaAccountSchema;
use crate::secondary::repositories::scylla::scylla_base_repository::ScyllaBaseRepository;

#[allow(dead_code)]
pub struct ScyllaAccountRepository {
    base: ScyllaBaseRepository<AccountEntity, ScyllaAccountSchema>,
}

impl ScyllaAccountRepository {
    pub fn new(session: Arc<Session>, keyspace: &str, table_name: &str) -> Self {
        ScyllaAccountRepository { base: ScyllaBaseRepository::new(session, keyspace, table_name) }
    }
}

impl_scylla_base_repository!(ScyllaAccountRepository, AccountEntity, ScyllaAccountSchema);

#[async_trait]
impl AccountRepository for ScyllaAccountRepository {
    async fn find_by_email(&self, email: &str) -> DomainResponse<Option<AccountEntity>> {
        let query = r#"
            SELECT id, avatar, bio, username, email, email_verified, is_active, created_at, updated_at, deleted_at
            FROM visionary.accounts
            WHERE email = ?
            ALLOW FILTERING
        "#;

        let prepared = self
            .base
            .session
            .prepare(query)
            .await
            .map_err(|e| Failure::DatabaseError(format!("Failed to prepare query: {}", e)))?;

        self.base
            .session
            .execute_unpaged(&prepared, (email,))
            .await
            .map_err(|e| Failure::DatabaseError(format!("Failed to find account by email: {}", e)))?;

        Ok(None)
    }
}
