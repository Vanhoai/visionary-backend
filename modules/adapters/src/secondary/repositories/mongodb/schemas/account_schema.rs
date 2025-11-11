use serde::{Deserialize, Serialize};

// shared modules
use domain::entities::account_entity::AccountEntity;

// internal modules
use crate::secondary::repositories::mongodb::mongo_base_repository::EntitySchema;
use crate::secondary::repositories::mongodb::schemas::base_schema::BaseSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSchema {
    #[serde(flatten)]
    pub base: BaseSchema,
    pub username: String,
    pub avatar: String,
    pub email: String,
    pub email_verified: bool,
    pub bio: String,
    pub is_active: bool,
}

impl EntitySchema<AccountEntity> for AccountSchema {
    fn from_entity(entity: AccountEntity) -> Self {
        Self {
            base: BaseSchema::from_entity(entity.base),
            username: entity.username,
            avatar: entity.avatar,
            email: entity.email,
            email_verified: entity.email_verified,
            bio: entity.bio,
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
