use async_trait::async_trait;
use domain::repositories::base_repository::BaseRepository;
use mongodb::Collection;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_document};
use serde::Serialize;
use serde::de::DeserializeOwned;
use shared::models::failure::Failure;
use shared::types::DomainResponse;
use std::marker::PhantomData;
use std::sync::Arc;

pub trait EntitySchema<S>: Serialize + DeserializeOwned + Unpin + Send + Sync {
    fn from_entity(entity: S) -> Self;
    fn to_entity(&self) -> S;
}

pub struct MongoBaseRepository<E, S>
where
    E: Send + Sync + Clone,
    S: EntitySchema<E>,
{
    pub collection: Arc<Collection<S>>,
    _phantom: PhantomData<E>,
}

impl<E, S> MongoBaseRepository<E, S>
where
    E: Send + Sync + Clone,
    S: EntitySchema<E>,
{
    pub fn new(collection: Arc<Collection<S>>) -> Self {
        MongoBaseRepository { collection, _phantom: PhantomData }
    }
}

#[macro_export]
macro_rules! impl_mongo_base_repository {
    ($repository:ty, $entity:ty, $schema:ty) => {
        #[async_trait::async_trait]
        impl domain::repositories::base_repository::BaseRepository<$entity> for $repository {
            async fn create(&self, entity: $entity) -> shared::types::DomainResponse<$entity> {
                self.base.create(entity).await
            }

            async fn update(&self, id: &str, entity: $entity) -> shared::types::DomainResponse<$entity> {
                self.base.update(id, entity).await
            }

            async fn delete(&self, id: &str) -> shared::types::DomainResponse<$entity> {
                self.base.delete(id).await
            }

            async fn remove(&self, id: &str) -> shared::types::DomainResponse<$entity> {
                self.base.remove(id).await
            }

            async fn find_by_id(&self, id: &str) -> shared::types::DomainResponse<Option<$entity>> {
                self.base.find_by_id(id).await
            }
        }
    };
}

#[async_trait]
impl<E, S> BaseRepository<E> for MongoBaseRepository<E, S>
where
    E: Send + Sync + Clone,
    S: EntitySchema<E>,
{
    async fn create(&self, entity: E) -> DomainResponse<E> {
        let schema = S::from_entity(entity.clone());
        let inserted_result = self
            .collection
            .insert_one(&schema)
            .await
            .map_err(|e| Failure::DatabaseError(format!("Failed to create entity: {}", e)))?;

        // Get the inserted _id and update entity
        let inserted_id = inserted_result
            .inserted_id
            .as_object_id()
            .ok_or_else(|| Failure::InternalError("Failed to get inserted ID".to_string()))?;

        // Fetch the inserted document to get the complete schema with _id
        let filter = doc! { "_id": inserted_id };

        // FIXME 🫠: Because I can't figure out how to set the ID back to entity after insertion,
        // so I have to fetch the created entity again.
        let schema = self
            .collection
            .find_one(filter)
            .await
            .map_err(|e| Failure::DatabaseError(format!("Failed to fetch created entity: {}", e)))?
            .ok_or_else(|| Failure::InternalError("Created entity not found".to_string()))?;

        // Convert schema back to entity
        Ok(schema.to_entity())
    }

    async fn update(&self, id: &str, entity: E) -> DomainResponse<E> {
        let object_id =
            ObjectId::parse_str(id).map_err(|_| Failure::BadRequest(format!("Invalid ID format: {}", id)))?;

        let filter = doc! { "_id": object_id };

        let schema = S::from_entity(entity.clone());

        let update = doc! {
            "$set": to_document(&schema)
                .map_err(|e| Failure::DatabaseError(format!("Failed to serialize entity for update: {}", e)))?,
        };

        let updated_schema = self
            .collection
            .find_one_and_update(filter, update)
            .await
            .map_err(|e| Failure::DatabaseError(format!("Failed to update entity: {}", e)))?
            .ok_or_else(|| Failure::NotFound(format!("Entity with id {} not found", id)))?;

        Ok(updated_schema.to_entity())
    }

    async fn delete(&self, id: &str) -> DomainResponse<E> {
        let object_id =
            ObjectId::parse_str(id).map_err(|_| Failure::BadRequest(format!("Invalid ID format: {}", id)))?;

        let filter = doc! { "_id": object_id };

        // Soft delete by setting deleted_at timestamp
        let update = doc! {
            "$set": {
                "deleted_at": chrono::Utc::now().timestamp(),
            }
        };

        let schema = self
            .collection
            .find_one_and_update(filter, update)
            .await
            .map_err(|e| Failure::DatabaseError(format!("Failed to delete entity: {}", e)))?
            .ok_or_else(|| Failure::NotFound(format!("Entity with id {} not found", id)))?;

        Ok(schema.to_entity())
    }

    async fn remove(&self, id: &str) -> DomainResponse<E> {
        let object_id =
            ObjectId::parse_str(id).map_err(|_| Failure::BadRequest(format!("Invalid ID format: {}", id)))?;

        let filter = doc! { "_id": object_id };

        let schema = self
            .collection
            .find_one_and_delete(filter)
            .await
            .map_err(|e| Failure::DatabaseError(format!("Failed to remove entity: {}", e)))?
            .ok_or_else(|| Failure::NotFound(format!("Entity with id {} not found", id)))?;

        Ok(schema.to_entity())
    }

    async fn find_by_id(&self, id: &str) -> DomainResponse<Option<E>> {
        let object_id =
            ObjectId::parse_str(id).map_err(|_| Failure::BadRequest(format!("Invalid ID format: {}", id)))?;

        let filter = doc! {
            "_id": object_id,
            "deleted_at": { "$exists": false }
        };

        match self.collection.find_one(filter).await {
            Ok(Some(schema)) => Ok(Some(schema.to_entity())),
            Ok(None) => Ok(None),
            Err(e) => Err(Failure::DatabaseError(format!("Failed to find entity by ID: {}", e))),
        }
    }
}
