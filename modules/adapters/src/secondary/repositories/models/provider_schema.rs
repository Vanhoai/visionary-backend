use mongodb::bson::oid::ObjectId;
use scylla::SerializeRow;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

// shared modules
use domain::entities::{base_entity::BaseEntity, provider_entity::ProviderEntity};

// internal modules
use crate::secondary::repositories::{
    models::base_schema::MongoBaseSchema, mongodb::mongo_base_repository, scylla::scylla_base_repository,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoProviderSchema {
    #[serde(flatten)]
    pub base: MongoBaseSchema,
    pub account_id: ObjectId,
    pub auth_provider: String,
    pub identify: String,
}

impl mongo_base_repository::EntitySchema<ProviderEntity> for MongoProviderSchema {
    fn from_entity(entity: &ProviderEntity) -> Self {
        Self {
            base: MongoBaseSchema::from_entity(&entity.base),
            account_id: ObjectId::from_str(entity.account_id.as_str()).unwrap(),
            auth_provider: entity.auth_provider.clone(),
            identify: entity.identify.clone(),
        }
    }

    fn to_entity(&self) -> ProviderEntity {
        ProviderEntity {
            base: self.base.to_entity(),
            account_id: self.account_id.to_string(),
            auth_provider: self.auth_provider.clone(),
            identify: self.identify.clone(),
        }
    }
}

#[derive(Debug, Clone, SerializeRow)]
pub struct ScyllaProviderSchema {
    pub id: Option<Uuid>,
    pub account_id: String,
    pub auth_provider: String,
    pub identify: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl scylla_base_repository::EntitySchema<ProviderEntity> for ScyllaProviderSchema {
    fn from_entity(entity: &ProviderEntity) -> Self {
        Self {
            id: entity.base.id.as_ref().and_then(|id| Uuid::parse_str(id).ok()),
            created_at: entity.base.created_at,
            updated_at: entity.base.updated_at,
            deleted_at: entity.base.deleted_at,
            account_id: entity.account_id.clone(),
            auth_provider: entity.auth_provider.clone(),
            identify: entity.identify.clone(),
        }
    }

    fn to_entity(&self) -> ProviderEntity {
        ProviderEntity {
            base: BaseEntity {
                id: self.id.as_ref().map(|id| id.to_string()),
                created_at: self.created_at,
                updated_at: self.updated_at,
                deleted_at: self.deleted_at,
            },
            account_id: self.account_id.clone(),
            auth_provider: self.auth_provider.clone(),
            identify: self.identify.clone(),
        }
    }

    fn columns() -> &'static str {
        "id, account_id, auth_provider, identify, created_at, updated_at, deleted_at"
    }

    fn insert_placeholders() -> &'static str {
        "?, ?, ?, ?, ?, ?, ?"
    }
}
