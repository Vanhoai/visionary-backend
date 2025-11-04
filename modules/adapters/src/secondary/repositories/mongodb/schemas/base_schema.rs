use domain::entities::base_entity::BaseEntity;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<i64>,
}

impl BaseSchema {
    pub fn new(id: Option<ObjectId>, created_at: i64, updated_at: i64, deleted_at: Option<i64>) -> Self {
        Self { id, created_at, updated_at, deleted_at }
    }

    pub fn from_entity(entity: BaseEntity) -> Self {
        BaseSchema::new(
            entity.id.as_ref().and_then(|id| ObjectId::from_str(id).ok()),
            entity.created_at,
            entity.updated_at,
            entity.deleted_at,
        )
    }

    pub fn to_entity(&self) -> BaseEntity {
        BaseEntity {
            id: self.id.as_ref().map(|oid| oid.to_hex()),
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        }
    }
}
