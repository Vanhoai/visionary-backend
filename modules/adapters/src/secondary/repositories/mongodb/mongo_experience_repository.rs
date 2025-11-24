use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

// shared modules
use domain::entities::experience_entity::ExperienceEntity;
use domain::repositories::experience_repository::ExperienceRepository;
use processors::MongoRepository;
use shared::models::failure::Failure;
use shared::types::DomainResponse;

// internal modules
use crate::secondary::repositories::{
    models::experience_schema::MongoExperienceSchema,
    mongodb::mongo_base_repository::{EntitySchema, MongoBaseRepository},
};

#[derive(MongoRepository)]
pub struct MongoExperienceRepository {
    base: MongoBaseRepository<ExperienceEntity, MongoExperienceSchema>,
}

impl MongoExperienceRepository {
    pub fn new(collection: Arc<Collection<MongoExperienceSchema>>) -> Self {
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
            .try_collect::<Vec<MongoExperienceSchema>>()
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

    async fn update_experience_partial(
        &self,
        id: &str,
        technologies: Option<Vec<String>>,
        position: Option<String>,
        responsibility: Option<Vec<String>>,
        company: Option<String>,
        location: Option<String>,
        start_date: Option<i64>,
        end_date: Option<Option<i64>>,
        is_current: Option<bool>,
    ) -> DomainResponse<ExperienceEntity> {
        let object_id =
            ObjectId::parse_str(id).map_err(|e| Failure::BadRequest(format!("Invalid id format: {}: {}", id, e)))?;

        // Build update document dynamically based on provided fields
        let mut update_doc = doc! {};

        if let Some(tech) = technologies {
            update_doc.insert("technologies", tech);
        }
        if let Some(pos) = position {
            update_doc.insert("position", pos);
        }
        if let Some(resp) = responsibility {
            update_doc.insert("responsibility", resp);
        }
        if let Some(comp) = company {
            update_doc.insert("company", comp);
        }
        if let Some(loc) = location {
            update_doc.insert("location", loc);
        }
        if let Some(start) = start_date {
            update_doc.insert("start_date", start);
        }

        if let Some(end) = end_date {
            match end {
                Some(end_val) => update_doc.insert("end_date", end_val),
                None => update_doc.insert("end_date", mongodb::bson::Bson::Null),
            };
        }
        if let Some(current) = is_current {
            update_doc.insert("is_current", current);
        }

        // If no fields to update, return error
        if update_doc.is_empty() {
            return Err(Failure::BadRequest("No fields provided for update".to_string()));
        }

        // update the updated_at timestamp
        let current_timestamp = chrono::Utc::now().timestamp_millis();
        update_doc.insert("updated_at", current_timestamp);

        let filter = doc! { "_id": object_id };
        let update = doc! { "$set": update_doc };

        self.base
            .collection
            .find_one_and_update(filter, update)
            .return_document(mongodb::options::ReturnDocument::After)
            .await
            .map_err(|e| Failure::BadRequest(format!("Mongo update error: {}", e)))?
            .map(|schema| schema.to_entity())
            .ok_or_else(|| Failure::NotFound(format!("Experience with id {} not found", id)))
    }
}
