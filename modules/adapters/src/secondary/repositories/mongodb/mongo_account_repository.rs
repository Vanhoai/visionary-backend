use async_trait::async_trait;
use domain::entities::account_entity::AccountEntity;
use domain::repositories::account_repository::AccountRepository;
use mongodb::Collection;
use std::sync::Arc;

pub struct MongoAccountRepository {
    pub collection: Arc<Collection<AccountEntity>>,
}

impl MongoAccountRepository {
    pub fn new(collection: Arc<Collection<AccountEntity>>) -> Self {
        MongoAccountRepository { collection }
    }
}

#[async_trait]
impl AccountRepository for MongoAccountRepository {}
