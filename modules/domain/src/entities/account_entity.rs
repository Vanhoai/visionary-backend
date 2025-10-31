use chrono::format::Numeric::Timestamp;
use serde::Serialize;
use shared::models::failure::Failure;
use uuid::Uuid;

use crate::entities::base_entity::BaseEntity;

#[derive(Debug, Clone, Serialize)]
pub struct AccountEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub username: String,
    pub avatar: String,
    pub email: String,
    pub email_verified: bool,
    pub bio: String,
    pub is_active: bool,
}

impl AccountEntity {
    const MAX_USERNAME_LENGTH: usize = 20;

    pub fn new(username: String, email: String) -> Result<Self, Failure> {
        Self::validate_username(&username)?;

        Ok(AccountEntity {
            base: BaseEntity::new(),
            username,
            email,
            avatar: String::new(),
            email_verified: false,
            bio: "Please write your bio.".to_string(),
            is_active: true,
        })
    }

    fn validate_username(username: &str) -> Result<(), Failure> {
        if username.len() > Self::MAX_USERNAME_LENGTH {
            return Err(Failure::ValidationError(format!(
                "Username must not exceed {} characters",
                Self::MAX_USERNAME_LENGTH
            )));
        }

        Ok(())
    }
}
