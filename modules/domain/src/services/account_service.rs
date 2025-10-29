use std::sync::Arc;
use async_trait::async_trait;
use crate::repositories::account_repository::AccountRepository;

#[async_trait]
pub trait AccountService: Send + Sync {}

pub struct AccountServiceImpl {
    repository: Arc<dyn AccountRepository>
}

impl AccountServiceImpl {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl AccountService for AccountServiceImpl {}
