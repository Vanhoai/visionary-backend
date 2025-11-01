use async_trait::async_trait;
use domain::entities::account_entity::AccountEntity;
use domain::repositories::account_repository::AccountRepository;
use shared::types::DomainResponse;

pub struct ScyllaAccountRepository {}

impl ScyllaAccountRepository {
    pub fn new() -> Self {
        ScyllaAccountRepository {}
    }
}

#[async_trait]
impl AccountRepository for ScyllaAccountRepository {
    async fn create(&self, account: AccountEntity) -> DomainResponse<AccountEntity> {
        todo!()
    }

    async fn find_by_email(&self, email: &str) -> DomainResponse<Option<AccountEntity>> {
        todo!()
    }
}
