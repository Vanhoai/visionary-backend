use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// shared modules
use domain::entities::provider_entity::ProviderEntity;

// internal modules
use crate::secondary::repositories::{
    models::base_schema::{MongoBaseSchema, ScyllaBaseSchema},
    mongodb::mongo_base_repository,
    scylla::scylla_base_repository,
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

#[derive(Debug, Clone)]
pub struct ScyllaProviderSchema {
    pub base: ScyllaBaseSchema,
    pub account_id: String,
    pub auth_provider: String,
    pub identify: String,
}

impl scylla_base_repository::EntitySchema<ProviderEntity> for ScyllaProviderSchema {
    fn from_entity(entity: &ProviderEntity) -> Self {
        Self {
            base: ScyllaBaseSchema::from_entity(&entity.base),
            account_id: entity.account_id.clone(),
            auth_provider: entity.auth_provider.clone(),
            identify: entity.identify.clone(),
        }
    }

    fn to_entity(&self) -> ProviderEntity {
        ProviderEntity {
            base: self.base.to_entity(),
            account_id: self.account_id.clone(),
            auth_provider: self.auth_provider.clone(),
            identify: self.identify.clone(),
        }
    }
}
