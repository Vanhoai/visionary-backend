use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// shared modules
use domain::entities::session_entity::SessionEntity;

// internal modules
use crate::secondary::repositories::mongodb::{mongo_base_repository::EntitySchema, schemas::base_schema::BaseSchema};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionSchema {
    #[serde(flatten)]
    pub base: BaseSchema,
    pub account_id: ObjectId,
    pub jit: String,
    pub expires_at: i64,
    pub ip_address: String,
    pub user_agent: String,
    pub device_type: String,
}

impl EntitySchema<SessionEntity> for SessionSchema {
    fn from_entity(entity: SessionEntity) -> Self {
        SessionSchema {
            base: BaseSchema::from_entity(entity.base),
            account_id: ObjectId::parse_str(&entity.account_id).unwrap(),
            jit: entity.jit,
            expires_at: entity.expires_at,
            ip_address: entity.ip_address,
            user_agent: entity.user_agent,
            device_type: entity.device_type,
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
