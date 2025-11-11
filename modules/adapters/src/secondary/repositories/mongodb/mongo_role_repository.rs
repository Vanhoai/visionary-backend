use async_trait::async_trait;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use std::sync::Arc;

// shared modules
use domain::entities::role_entity::RoleEntity;
use domain::repositories::role_repository::RoleRepository;
use shared::models::failure::Failure;
use shared::types::DomainResponse;

// internal modules
use crate::impl_mongo_base_repository;
use crate::secondary::repositories::mongodb::mongo_base_repository::{EntitySchema, MongoBaseRepository};
use crate::secondary::repositories::mongodb::schemas::role_schema::RoleSchema;

pub struct MongoRoleRepository {
    base: MongoBaseRepository<RoleEntity, RoleSchema>,
}

impl MongoRoleRepository {
    pub fn new(collection: Arc<Collection<RoleSchema>>) -> Self {
        MongoRoleRepository { base: MongoBaseRepository::new(collection) }
    }
}

impl_mongo_base_repository!(MongoRoleRepository, RoleEntity, RoleSchema);

#[async_trait]
impl RoleRepository for MongoRoleRepository {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Option<RoleEntity>> {
        let object_id = ObjectId::parse_str(account_id)
            .map_err(|e| Failure::BadRequest(format!("Invalid account_id format: {}: {}", account_id, e)))?;
        let filter = doc! { "account_id": object_id };

        match self.base.collection.find_one(filter).await {
            Ok(Some(schema)) => Ok(Some(schema.to_entity())),
            Ok(None) => Ok(None),
            Err(e) => Err(Failure::DatabaseError(format!("Failed to find role by account_id: {}", e))),
        }
    }

    async fn find_and_update_role_by_account_id(
        &self,
        account_id: &str,
        role_name: &str,
    ) -> DomainResponse<RoleEntity> {
        let object_id = ObjectId::parse_str(account_id)
            .map_err(|e| Failure::BadRequest(format!("Invalid account_id format: {}: {}", account_id, e)))?;

        let filter = doc! { "account_id": object_id };
        let update = doc! { "$set": { "role_name": role_name } };

        match self.base.collection.find_one_and_update(filter, update).await {
            Ok(Some(updated_schema)) => Ok(updated_schema.to_entity()),
            Ok(None) => Err(Failure::NotFound(format!("Role not found for account_id: {}", account_id))),
            Err(e) => Err(Failure::DatabaseError(format!("Failed to update role by account_id: {}", e))),
        }
    }
}
