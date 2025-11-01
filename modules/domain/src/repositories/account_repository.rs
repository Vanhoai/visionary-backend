use crate::entities::account_entity::AccountEntity;
use async_trait::async_trait;
use shared::models::failure::Failure;
use shared::types::DomainResponse;

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn create(&self, account: AccountEntity) -> DomainResponse<AccountEntity>;
    async fn find_by_email(&self, email: &str) -> DomainResponse<Option<AccountEntity>>;
}
