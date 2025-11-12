use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

// shared modules
use domain::entities::provider_entity::ProviderEntity;
use domain::repositories::provider_repository::ProviderRepository;
use shared::models::failure::Failure;
use shared::types::DomainResponse;

// internal modules
use crate::impl_mongo_base_repository;
use crate::secondary::repositories::models::provider_schema::MongoProviderSchema;
use crate::secondary::repositories::mongodb::mongo_base_repository::{EntitySchema, MongoBaseRepository};

pub struct MongoProviderRepository {
    base: MongoBaseRepository<ProviderEntity, MongoProviderSchema>,
}

impl MongoProviderRepository {
    pub fn new(collection: Arc<Collection<MongoProviderSchema>>) -> Self {
        MongoProviderRepository { base: MongoBaseRepository::new(collection) }
    }
}

impl_mongo_base_repository!(MongoProviderRepository, ProviderEntity, MongoProviderSchema);

#[async_trait]
impl ProviderRepository for MongoProviderRepository {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<ProviderEntity>> {
        let object_id = ObjectId::parse_str(account_id)
            .map_err(|e| Failure::BadRequest(format!("Invalid account_id format: {}: {}", account_id, e)))?;

        let filter = doc! { "account_id": object_id };
        let cursor = self
            .base
            .collection
            .find(filter)
            .await
            .map_err(|e| Failure::BadRequest(format!("Mongo search error: {}", e)))?;

        let entities = cursor
            .try_collect::<Vec<MongoProviderSchema>>()
            .await
            .map_err(|e| Failure::BadRequest(format!("Failed to collect documents: {}", e)))?
            .into_iter()
            .map(|schema| schema.to_entity())
            .collect();

        Ok(entities)
    }
}
