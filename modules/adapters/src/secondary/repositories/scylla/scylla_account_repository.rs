use async_trait::async_trait;
use domain::repositories::account_repository::AccountRepository;

pub struct ScyllaAccountRepository {}

impl ScyllaAccountRepository {
    pub fn new() -> Self {
        ScyllaAccountRepository {}
    }
}

#[async_trait]
impl AccountRepository for ScyllaAccountRepository {}
