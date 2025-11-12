use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// shared modules
use domain::entities::session_entity::SessionEntity;

// internal modules
use crate::secondary::repositories::{
    models::base_schema::{MongoBaseSchema, ScyllaBaseSchema},
    mongodb::mongo_base_repository,
    scylla::scylla_base_repository,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MongoSessionSchema {
    #[serde(flatten)]
    pub base: MongoBaseSchema,
    pub account_id: ObjectId,
    pub jit: String,
    pub expires_at: i64,
    pub ip_address: String,
    pub user_agent: String,
    pub device_type: String,
}

impl mongo_base_repository::EntitySchema<SessionEntity> for MongoSessionSchema {
    fn from_entity(entity: &SessionEntity) -> Self {
        MongoSessionSchema {
            base: MongoBaseSchema::from_entity(&entity.base),
            account_id: ObjectId::parse_str(&entity.account_id).unwrap(),
            jit: entity.jit.clone(),
            expires_at: entity.expires_at,
            ip_address: entity.ip_address.clone(),
            user_agent: entity.user_agent.clone(),
            device_type: entity.device_type.clone(),
        }
    }

    fn to_entity(&self) -> SessionEntity {
        SessionEntity {
            base: self.base.to_entity(),
            account_id: self.account_id.to_hex(),
            jit: self.jit.clone(),
            expires_at: self.expires_at,
            ip_address: self.ip_address.clone(),
            user_agent: self.user_agent.clone(),
            device_type: self.device_type.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScyllaSessionSchema {
    pub base: ScyllaBaseSchema,
    pub account_id: String,
    pub jit: String,
    pub expires_at: i64,
    pub ip_address: String,
    pub user_agent: String,
    pub device_type: String,
}

impl scylla_base_repository::EntitySchema<SessionEntity> for ScyllaSessionSchema {
    fn from_entity(entity: &SessionEntity) -> Self {
        ScyllaSessionSchema {
            base: ScyllaBaseSchema::from_entity(&entity.base),
            account_id: entity.account_id.clone(),
            jit: entity.jit.clone(),
            expires_at: entity.expires_at,
            ip_address: entity.ip_address.clone(),
            user_agent: entity.user_agent.clone(),
            device_type: entity.device_type.clone(),
        }
    }

    fn to_entity(&self) -> SessionEntity {
        SessionEntity {
            base: self.base.to_entity(),
            account_id: self.account_id.clone(),
            jit: self.jit.clone(),
            expires_at: self.expires_at,
            ip_address: self.ip_address.clone(),
            user_agent: self.user_agent.clone(),
            device_type: self.device_type.clone(),
        }
    }
}
