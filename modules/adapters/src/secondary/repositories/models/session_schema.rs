use mongodb::bson::oid::ObjectId;
use scylla::SerializeRow;
use serde::{Deserialize, Serialize};

// shared modules
use domain::entities::{base_entity::BaseEntity, session_entity::SessionEntity};
use uuid::Uuid;

// internal modules
use crate::secondary::repositories::{
    models::base_schema::MongoBaseSchema, mongodb::mongo_base_repository, scylla::scylla_base_repository,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MongoSessionSchema {
    #[serde(flatten)]
    pub base: MongoBaseSchema,
    pub account_id: ObjectId,
    pub jti: String,
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
            jti: entity.jti.clone(),
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
            jti: self.jti.clone(),
            expires_at: self.expires_at,
            ip_address: self.ip_address.clone(),
            user_agent: self.user_agent.clone(),
            device_type: self.device_type.clone(),
        }
    }
}

#[derive(Debug, Clone, SerializeRow)]
pub struct ScyllaSessionSchema {
    pub id: Option<Uuid>,
    pub account_id: String,
    pub jti: String,
    pub expires_at: i64,
    pub ip_address: String,
    pub user_agent: String,
    pub device_type: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl scylla_base_repository::EntitySchema<SessionEntity> for ScyllaSessionSchema {
    fn from_entity(entity: &SessionEntity) -> Self {
        ScyllaSessionSchema {
            id: entity.base.id.as_ref().and_then(|id| Uuid::parse_str(id).ok()),
            created_at: entity.base.created_at,
            updated_at: entity.base.updated_at,
            deleted_at: entity.base.deleted_at,
            account_id: entity.account_id.clone(),
            jti: entity.jti.clone(),
            expires_at: entity.expires_at,
            ip_address: entity.ip_address.clone(),
            user_agent: entity.user_agent.clone(),
            device_type: entity.device_type.clone(),
        }
    }

    fn to_entity(&self) -> SessionEntity {
        SessionEntity {
            base: BaseEntity {
                id: self.id.as_ref().map(|id| id.to_string()),
                created_at: self.created_at,
                updated_at: self.updated_at,
                deleted_at: self.deleted_at,
            },
            account_id: self.account_id.clone(),
            jti: self.jti.clone(),
            expires_at: self.expires_at,
            ip_address: self.ip_address.clone(),
            user_agent: self.user_agent.clone(),
            device_type: self.device_type.clone(),
        }
    }

    fn columns() -> &'static str {
        "id, account_id, jti, expires_at, ip_address, user_agent, device_type, created_at, updated_at, deleted_at"
    }

    fn insert_placeholders() -> &'static str {
        "?, ?, ?, ?, ?, ?, ?, ?, ?, ?"
    }
}
