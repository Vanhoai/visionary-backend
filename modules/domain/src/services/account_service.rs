use crate::entities::account_entity::AccountEntity;
use crate::repositories::account_repository::AccountRepository;
use async_trait::async_trait;
use shared::types::DomainResponse;
use std::sync::Arc;

#[async_trait]
pub trait AccountService: Send + Sync {
    async fn check_email_exists(&self, email: String) -> DomainResponse<bool>;
    async fn create_account(&self, username: String, email: String) -> DomainResponse<AccountEntity>;
    async fn find_by_email(&self, email: String) -> DomainResponse<Option<AccountEntity>>;
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
    async fn check_email_exists(&self, email: String) -> DomainResponse<bool> {
        match self.repository.find_by_email(&email).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn create_account(&self, username: String, email: String) -> DomainResponse<AccountEntity> {
        let account = AccountEntity::new(false, username, email)?;
        self.repository.create(account).await
    }

    async fn find_by_email(&self, email: String) -> DomainResponse<Option<AccountEntity>> {
        self.repository.find_by_email(&email).await
    }
}
