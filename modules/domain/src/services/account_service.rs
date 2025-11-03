use crate::entities::account_entity::AccountEntity;
use crate::repositories::account_repository::AccountRepository;
use async_trait::async_trait;
use shared::models::failure::Failure;
use std::sync::Arc;

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
        match self.repository.find_by_email(email).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn create_account(&self, username: String, email: String) -> Result<AccountEntity, Failure> {
        // Business logic: validate & create account will be here
        let account = AccountEntity::new(username, email)?;

        // Save to repository
        self.repository.create(account).await
    }
}
