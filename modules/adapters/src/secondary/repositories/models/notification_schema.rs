use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// shared modules
use domain::entities::notification_entity::NotificationEntity;
use uuid::Uuid;

// internal modules
use crate::secondary::repositories::{
    models::base_schema::{MongoBaseSchema, ScyllaBaseSchema},
    mongodb::mongo_base_repository,
    scylla::scylla_base_repository,
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

#[derive(Clone, Debug)]
pub struct ScyllaNotificationSchema {
    pub base: ScyllaBaseSchema,
    pub account_id: Uuid,
    pub message: String,
    pub is_read: bool,
}

impl scylla_base_repository::EntitySchema<NotificationEntity> for ScyllaNotificationSchema {
    fn from_entity(entity: &NotificationEntity) -> Self {
        Self {
            base: ScyllaBaseSchema::from_entity(&entity.base),
            account_id: Uuid::parse_str(&entity.account_id).unwrap(),
            message: entity.message.clone(),
            is_read: entity.is_read,
        }
    }

    fn to_entity(&self) -> NotificationEntity {
        NotificationEntity {
            base: self.base.to_entity(),
            account_id: self.account_id.to_string(),
            message: self.message.clone(),
            is_read: self.is_read,
        }
    }
}
