use crate::impl_mongo_base_repository;
use crate::secondary::repositories::mongodb::mongo_base_repository::{EntitySchema, MongoBaseRepository};
use crate::secondary::repositories::mongodb::schemas::work_schema::WorkSchema;
use async_trait::async_trait;
use domain::entities::work_entity::WorkEntity;
use domain::repositories::work_repository::WorkRepository;
use futures::TryStreamExt;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use shared::models::failure::Failure;
use shared::types::DomainResponse;
use std::sync::Arc;

pub struct MongoWorkRepository {
    base: MongoBaseRepository<WorkEntity, WorkSchema>,
}

impl MongoWorkRepository {
    pub fn new(collection: Arc<Collection<WorkSchema>>) -> Self {
        MongoWorkRepository { base: MongoBaseRepository::new(collection) }
    }
}

impl_mongo_base_repository!(MongoWorkRepository, WorkEntity, WorkSchema);

#[async_trait]
impl WorkRepository for MongoWorkRepository {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<WorkEntity>> {
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
            .try_collect::<Vec<WorkSchema>>()
            .await
            .map_err(|e| Failure::BadRequest(format!("Failed to collect documents: {}", e)))?
            .into_iter()
            .map(|schema| schema.to_entity())
            .collect();

        Ok(entities)
    }
}
