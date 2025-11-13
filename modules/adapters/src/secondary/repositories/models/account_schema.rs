use domain::entities::base_entity::BaseEntity;
use scylla::SerializeRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// shared modules
use crate::secondary::repositories::models::base_schema::MongoBaseSchema;
use domain::entities::account_entity::AccountEntity;

// internal modules
use crate::secondary::repositories::mongodb::mongo_base_repository;
use crate::secondary::repositories::scylla::scylla_base_repository;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountMongoSchema {
    #[serde(flatten)]
    pub base: MongoBaseSchema,
    pub username: String,
    pub avatar: String,
    pub email: String,
    pub email_verified: bool,
    pub bio: String,
    pub is_active: bool,
}

impl mongo_base_repository::EntitySchema<AccountEntity> for AccountMongoSchema {
    fn from_entity(entity: &AccountEntity) -> Self {
        Self {
            base: MongoBaseSchema::from_entity(&entity.base),
            username: entity.username.clone(),
            avatar: entity.avatar.clone(),
            email: entity.email.clone(),
            email_verified: entity.email_verified,
            bio: entity.bio.clone(),
            is_active: entity.is_active,
        }
    }

    fn to_entity(&self) -> AccountEntity {
        AccountEntity {
            base: self.base.to_entity(),
            username: self.username.clone(),
            avatar: self.avatar.clone(),
            email: self.email.clone(),
            email_verified: self.email_verified,
            bio: self.bio.clone(),
            is_active: self.is_active,
        }
    }
}

#[derive(Debug, Clone, SerializeRow)]
pub struct ScyllaAccountSchema {
    pub id: Option<Uuid>,
    pub username: String,
    pub avatar: String,
    pub email: String,
    pub email_verified: bool,
    pub bio: String,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl scylla_base_repository::EntitySchema<AccountEntity> for ScyllaAccountSchema {
    fn from_entity(entity: &AccountEntity) -> Self {
        Self {
            id: entity.base.id.as_ref().and_then(|id| Uuid::parse_str(id).ok()),
            username: entity.username.clone(),
            avatar: entity.avatar.clone(),
            email: entity.email.clone(),
            email_verified: entity.email_verified,
            bio: entity.bio.clone(),
            is_active: entity.is_active,
            created_at: entity.base.created_at,
            updated_at: entity.base.updated_at,
            deleted_at: entity.base.deleted_at,
        }
    }

    fn to_entity(&self) -> AccountEntity {
        AccountEntity {
            base: BaseEntity {
                id: self.id.map(|id| id.to_string()),
                created_at: self.created_at,
                updated_at: self.updated_at,
                deleted_at: self.deleted_at,
            },
            username: self.username.clone(),
            avatar: self.avatar.clone(),
            email: self.email.clone(),
            email_verified: self.email_verified,
            bio: self.bio.clone(),
            is_active: self.is_active,
        }
    }

    fn columns() -> &'static str {
        "id, username, avatar, email, email_verified, bio, is_active, created_at, updated_at, deleted_at"
    }

    fn insert_placeholders() -> &'static str {
        "?, ?, ?, ?, ?, ?, ?, ?, ?, ?"
    }
}
