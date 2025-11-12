use async_trait::async_trait;
use mongodb::Collection;
use mongodb::bson::doc;
use std::sync::Arc;

// shared modules
use domain::entities::account_entity::AccountEntity;
use domain::repositories::account_repository::AccountRepository;
use shared::models::failure::Failure;
use shared::types::DomainResponse;

// internal modules
use crate::impl_mongo_base_repository;
use crate::secondary::repositories::models::account_schema::AccountMongoSchema;
use crate::secondary::repositories::mongodb::mongo_base_repository::{EntitySchema, MongoBaseRepository};

pub struct MongoAccountRepository {
    base: MongoBaseRepository<AccountEntity, AccountMongoSchema>,
}

impl MongoAccountRepository {
    pub fn new(collection: Arc<Collection<AccountMongoSchema>>) -> Self {
        Self { base: MongoBaseRepository::new(collection) }
    }
}

// Delegate base methods to MongoBaseRepository
impl_mongo_base_repository!(MongoAccountRepository, AccountEntity, AccountMongoSchema);

#[async_trait]
impl AccountRepository for MongoAccountRepository {
    async fn find_by_email(&self, email: &str) -> DomainResponse<Option<AccountEntity>> {
        let filter = doc! {
            "email": email,
            "deleted_at": { "$exists": false }
        };

        match self.base.collection.find_one(filter).await {
            Ok(Some(schema)) => Ok(Some(schema.to_entity())),
            Ok(None) => Ok(None),
            Err(e) => Err(Failure::DatabaseError(format!("Failed to find account by email: {}", e))),
        }
    }
}
