use async_trait::async_trait;
use domain::repositories::account_repository::AccountRepository;

pub struct AccountRepositoryImpl {}

impl AccountRepositoryImpl {
    pub fn new() -> Self {
        AccountRepositoryImpl {}
    }
}

#[async_trait]
impl AccountRepository for AccountRepositoryImpl {}
