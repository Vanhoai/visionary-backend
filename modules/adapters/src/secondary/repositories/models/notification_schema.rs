use mongodb::bson::oid::ObjectId;
use scylla::SerializeRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// shared modules
use domain::entities::{base_entity::BaseEntity, notification_entity::NotificationEntity};

// internal modules
use crate::secondary::repositories::{
    models::base_schema::MongoBaseSchema, mongodb::mongo_base_repository, scylla::scylla_base_repository,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct MongoNotificationSchema {
    #[serde(flatten)]
    base: MongoBaseSchema,
    pub account_id: ObjectId,
    pub message: String,
    pub is_read: bool,
}

impl mongo_base_repository::EntitySchema<NotificationEntity> for MongoNotificationSchema {
    fn from_entity(entity: &NotificationEntity) -> Self {
        Self {
            base: MongoBaseSchema::from_entity(&entity.base),
            account_id: ObjectId::parse_str(&entity.account_id).unwrap(),
            message: entity.message.clone(),
            is_read: entity.is_read,
        }
    }

    fn to_entity(&self) -> NotificationEntity {
        NotificationEntity {
            base: self.base.to_entity(),
            account_id: self.account_id.to_hex(),
            message: self.message.clone(),
            is_read: self.is_read,
        }
    }
}

#[derive(Clone, Debug, SerializeRow)]
pub struct ScyllaNotificationSchema {
    pub id: Option<Uuid>,
    pub account_id: Uuid,
    pub message: String,
    pub is_read: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl scylla_base_repository::EntitySchema<NotificationEntity> for ScyllaNotificationSchema {
    fn from_entity(entity: &NotificationEntity) -> Self {
        Self {
            id: entity.base.id.as_ref().and_then(|id| Uuid::parse_str(id).ok()),
            account_id: Uuid::parse_str(&entity.account_id).unwrap(),
            message: entity.message.clone(),
            is_read: entity.is_read,
            created_at: entity.base.created_at,
            updated_at: entity.base.updated_at,
            deleted_at: entity.base.deleted_at,
        }
    }

    fn to_entity(&self) -> NotificationEntity {
        NotificationEntity {
            base: BaseEntity {
                id: self.id.as_ref().map(|id| id.to_string()),
                created_at: self.created_at,
                updated_at: self.updated_at,
                deleted_at: self.deleted_at,
            },
            account_id: self.account_id.to_string(),
            message: self.message.clone(),
            is_read: self.is_read,
        }
    }

    fn columns() -> &'static str {
        "id, account_id, message, is_read, created_at, updated_at, deleted_at"
    }

    fn insert_placeholders() -> &'static str {
        "?, ?, ?, ?, ?, ?, ?"
    }
}
