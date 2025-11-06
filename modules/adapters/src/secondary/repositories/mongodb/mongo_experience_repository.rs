use crate::impl_mongo_base_repository;
use crate::secondary::repositories::mongodb::mongo_base_repository::{EntitySchema, MongoBaseRepository};
use crate::secondary::repositories::mongodb::schemas::experience_schema::ExperienceSchema;
use async_trait::async_trait;
use domain::entities::experience_entity::ExperienceEntity;
use domain::repositories::experience_repository::ExperienceRepository;
use futures::TryStreamExt;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use processors::MongoRepository;
use shared::models::failure::Failure;
use shared::types::DomainResponse;
use std::sync::Arc;

#[derive(MongoRepository)]
pub struct MongoExperienceRepository {
    base: MongoBaseRepository<ExperienceEntity, ExperienceSchema>,
}

impl MongoExperienceRepository {
    pub fn new(collection: Arc<Collection<ExperienceSchema>>) -> Self {
        MongoExperienceRepository { base: MongoBaseRepository::new(collection) }
    }
}

#[async_trait]
impl ExperienceRepository for MongoExperienceRepository {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<ExperienceEntity>> {
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
            .try_collect::<Vec<ExperienceSchema>>()
            .await
            .map_err(|e| Failure::BadRequest(format!("Failed to collect documents: {}", e)))?
            .into_iter()
            .map(|schema| schema.to_entity())
            .collect();

        Ok(entities)
    }

    async fn find_by_company(&self, company: &str) -> DomainResponse<Option<ExperienceEntity>> {
        let filter = doc! { "company": company };
        let result = self
            .base
            .collection
            .find_one(filter)
            .await
            .map_err(|e| Failure::BadRequest(format!("Mongo search error: {}", e)))?;

        match result {
            Some(schema) => Ok(Some(schema.to_entity())),
            None => Ok(None),
        }
    }
}
