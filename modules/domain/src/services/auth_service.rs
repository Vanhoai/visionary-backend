use crate::repositories::account_repository::AccountRepository;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait AuthService: Send + Sync {}

pub struct AuthServiceImpl {
    repository: Arc<dyn AccountRepository>,
}

impl AuthServiceImpl {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {}
