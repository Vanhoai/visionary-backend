use crate::secondary::repositories::mongodb::mongo_base_repository::EntitySchema;
use domain::entities::account_entity::AccountEntity;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSchema {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub avatar: String,
    pub email: String,
    pub email_verified: bool,
    pub bio: String,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<i64>,
}

impl EntitySchema<AccountEntity> for AccountSchema {
    fn from_entity(entity: AccountEntity) -> Self {
        Self {
            id: entity.base.id.as_ref().and_then(|id| ObjectId::from_str(id).ok()),
            username: entity.username,
            avatar: entity.avatar,
            email: entity.email,
            email_verified: entity.email_verified,
            bio: entity.bio,
            is_active: entity.is_active,
            created_at: entity.base.created_at,
            updated_at: entity.base.updated_at,
            deleted_at: entity.base.deleted_at,
        }
    }

    fn to_entity(&self) -> AccountEntity {
        AccountEntity {
            base: domain::entities::base_entity::BaseEntity {
                id: self.id.as_ref().map(|oid| oid.to_hex()),
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
}
