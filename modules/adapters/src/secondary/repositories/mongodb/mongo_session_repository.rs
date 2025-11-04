use crate::impl_mongo_base_repository;
use crate::secondary::repositories::mongodb::mongo_base_repository::{EntitySchema, MongoBaseRepository};
use crate::secondary::repositories::mongodb::schemas::session_schema::SessionSchema;
use async_trait::async_trait;
use domain::{entities::session_entity::SessionEntity, repositories::session_repository::SessionRepository};
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use shared::{models::failure::Failure, types::DomainResponse};
use std::sync::Arc;

pub struct MongoSessionRepository {
    base: MongoBaseRepository<SessionEntity, SessionSchema>,
}

impl MongoSessionRepository {
    pub fn new(collection: Arc<Collection<SessionSchema>>) -> Self {
        MongoSessionRepository { base: MongoBaseRepository::new(collection) }
    }
}

impl_mongo_base_repository!(MongoSessionRepository, SessionEntity, SessionSchema);

#[async_trait]
impl SessionRepository for MongoSessionRepository {
    async fn remove_by_account_id(&self, account_id: &str) -> DomainResponse<()> {
        let object_id = ObjectId::parse_str(account_id)
            .map_err(|_| Failure::BadRequest(format!("Invalid ID format: {}", account_id)))?;
        let query = doc! { "account_id": object_id };

        self.base.collection.delete_many(query).await.map_err(|e| {
            Failure::DatabaseError(format!("Failed to remove session by account id with {:?}", e).to_string())
        })?;

        Ok(())
    }

    async fn find_by_jit(&self, jit: &str) -> DomainResponse<Option<SessionEntity>> {
        let query = doc! { "jit": jit };

        let result =
            self.base.collection.find_one(query).await.map_err(|e| {
                Failure::DatabaseError(format!("Failed to find session by jit with {:?}", e).to_string())
            })?;

        Ok(result.map(|schema| schema.to_entity()))
    }
}
