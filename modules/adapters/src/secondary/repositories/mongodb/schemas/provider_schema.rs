use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// shared modules
use domain::entities::provider_entity::ProviderEntity;

// internal modules
use crate::secondary::repositories::mongodb::mongo_base_repository::EntitySchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub account_id: ObjectId,
    pub auth_provider: String,
    pub identify: String,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<i64>,
}

impl EntitySchema<ProviderEntity> for ProviderSchema {
    fn from_entity(entity: ProviderEntity) -> Self {
        Self {
            id: entity.base.id.as_ref().and_then(|id| ObjectId::from_str(id.as_str()).ok()),
            account_id: ObjectId::from_str(entity.account_id.as_str()).unwrap(),
            auth_provider: entity.auth_provider,
            identify: entity.identify,
            created_at: entity.base.created_at,
            updated_at: entity.base.updated_at,
            deleted_at: entity.base.deleted_at,
        }
    }

    fn to_entity(&self) -> ProviderEntity {
        ProviderEntity {
            base: domain::entities::base_entity::BaseEntity {
                id: self.id.as_ref().map(|oid| oid.to_hex()),
                created_at: self.created_at,
                updated_at: self.updated_at,
                deleted_at: self.deleted_at,
            },
            account_id: self.account_id.to_string(),
            auth_provider: self.auth_provider.clone(),
            identify: self.identify.clone(),
        }
    }
}
