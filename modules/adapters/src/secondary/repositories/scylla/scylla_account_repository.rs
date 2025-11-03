use async_trait::async_trait;
use domain::entities::account_entity::AccountEntity;
use domain::repositories::account_repository::AccountRepository;
use domain::repositories::base_repository::BaseRepository;
use shared::types::DomainResponse;

pub struct ScyllaAccountRepository {}

impl ScyllaAccountRepository {
    pub fn new() -> Self {
        ScyllaAccountRepository {}
    }
}

#[async_trait]
impl BaseRepository<AccountEntity> for ScyllaAccountRepository {
    async fn create(&self, entity: AccountEntity) -> DomainResponse<AccountEntity> {
        todo!()
    }

    async fn update(&self, id: &str, entity: AccountEntity) -> DomainResponse<AccountEntity> {
        todo!()
    }

    async fn delete(&self, id: &str) -> DomainResponse<AccountEntity> {
        todo!()
    }

    async fn find_by_id(&self, id: &str) -> DomainResponse<Option<AccountEntity>> {
        todo!()
    }
}

#[async_trait]
impl AccountRepository for ScyllaAccountRepository {
    async fn find_by_email(&self, email: &str) -> DomainResponse<Option<AccountEntity>> {
        todo!()
    }
}
