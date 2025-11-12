use serde::{Deserialize, Serialize};

// shared modules
use crate::secondary::repositories::models::base_schema::{MongoBaseSchema, ScyllaBaseSchema};
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

#[derive(Debug, Clone)]
pub struct ScyllaAccountSchema {
    pub base: ScyllaBaseSchema,
    pub username: String,
    pub avatar: String,
    pub email: String,
    pub email_verified: bool,
    pub bio: String,
    pub is_active: bool,
}

impl scylla_base_repository::EntitySchema<AccountEntity> for ScyllaAccountSchema {
    fn from_entity(entity: &AccountEntity) -> Self {
        Self {
            base: ScyllaBaseSchema::from_entity(&entity.base),
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
