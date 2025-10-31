use crate::repositories::account_repository::AccountRepository;
use async_trait::async_trait;
use std::sync::Arc;
use shared::models::failure::Failure;
use crate::entities::account_entity::AccountEntity;

#[async_trait]
pub trait AccountService: Send + Sync {
    async fn check_email_exists(&self, email: &str) -> Result<bool, Failure>;
    async fn create_account(&self, username: String, email: String) -> Result<AccountEntity, Failure>;
}

pub struct AccountServiceImpl {
    repository: Arc<dyn AccountRepository>,
}

impl AccountServiceImpl {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl AccountService for AccountServiceImpl {
    async fn check_email_exists(&self, email: &str) -> Result<bool, Failure> {
        todo!()
    }

    async fn create_account(&self, username: String, email: String) -> Result<AccountEntity, Failure> {
        todo!()
    }
}
