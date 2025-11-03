use crate::{entities::account_entity::AccountEntity, repositories::base_repository::BaseRepository};
use async_trait::async_trait;
use shared::types::DomainResponse;

#[async_trait]
pub trait AccountRepository: BaseRepository<AccountEntity> {
    async fn find_by_email(&self, email: &str) -> DomainResponse<Option<AccountEntity>>;
}
