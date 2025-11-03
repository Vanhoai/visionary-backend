use crate::impl_mongo_base_repository;
use crate::secondary::repositories::mongodb::mongo_base_repository::{EntitySchema, MongoBaseRepository};
use crate::secondary::repositories::mongodb::schemas::provider_schema::ProviderSchema;
use async_trait::async_trait;
use domain::entities::provider_entity::ProviderEntity;
use domain::repositories::provider_repository::ProviderRepository;
use futures::TryStreamExt;
use mongodb::Collection;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{bson, doc, from_document};
use shared::models::failure::Failure;
use shared::types::DomainResponse;
use std::sync::Arc;

pub struct MongoProviderRepository {
    base: MongoBaseRepository<ProviderEntity, ProviderSchema>,
}

impl MongoProviderRepository {
    pub fn new(collection: Arc<Collection<ProviderSchema>>) -> Self {
        MongoProviderRepository { base: MongoBaseRepository::new(collection) }
    }
}

impl_mongo_base_repository!(MongoProviderRepository, ProviderEntity, ProviderSchema);

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
            .try_collect::<Vec<ProviderSchema>>()
            .await
            .map_err(|e| Failure::BadRequest(format!("Failed to collect documents: {}", e)))?
            .into_iter()
            .map(|schema| schema.to_entity())
            .collect();

        Ok(entities)
    }
}
