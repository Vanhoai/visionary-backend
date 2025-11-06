use crate::entities::account_entity::AccountEntity;
use crate::repositories::account_repository::AccountRepository;
use async_trait::async_trait;
use shared::models::paginate::Paginate;
use shared::types::DomainResponse;
use std::sync::Arc;

#[async_trait]
pub trait AccountService: Send + Sync {
    async fn check_email_exists(&self, email: &str) -> DomainResponse<bool>;
    async fn create_account(&self, username: &str, email: &str) -> DomainResponse<AccountEntity>;
    async fn find_by_email(&self, email: &str) -> DomainResponse<Option<AccountEntity>>;
    async fn find_account_by_id(&self, account_id: &str) -> DomainResponse<Option<AccountEntity>>;
    async fn find_accounts_paginated(
        &self,
        page: u32,
        page_size: u32,
    ) -> DomainResponse<(Paginate, Vec<AccountEntity>)>;
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
    async fn check_email_exists(&self, email: &str) -> DomainResponse<bool> {
        match self.repository.find_by_email(email).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn create_account(&self, username: &str, email: &str) -> DomainResponse<AccountEntity> {
        let account = AccountEntity::new(false, username.to_string(), email.to_string())?;
        self.repository.create(&account).await
    }

    async fn find_by_email(&self, email: &str) -> DomainResponse<Option<AccountEntity>> {
        self.repository.find_by_email(email).await
    }

    async fn find_account_by_id(&self, account_id: &str) -> DomainResponse<Option<AccountEntity>> {
        self.repository.find(account_id).await
    }

    async fn find_accounts_paginated(
        &self,
        page: u32,
        page_size: u32,
    ) -> DomainResponse<(Paginate, Vec<AccountEntity>)> {
        self.repository.finds_paginated(page, page_size).await
    }
}
