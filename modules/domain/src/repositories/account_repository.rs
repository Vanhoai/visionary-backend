use async_trait::async_trait;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::{entities::account_entity::AccountEntity, repositories::base_repository::BaseRepository};

#[async_trait]
pub trait AccountRepository: BaseRepository<AccountEntity> {
    async fn find_by_email(&self, email: &str) -> DomainResponse<Option<AccountEntity>>;
}
