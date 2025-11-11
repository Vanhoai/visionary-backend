use regex::Regex;
use serde::{Deserialize, Serialize};

// shared modules
use shared::models::failure::Failure;

// internal modules
use crate::entities::base_entity::BaseEntity;

const MAX_USERNAME_LENGTH: usize = 20;
const EMAIL_REGEX: &str = r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,6}$";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
    pub fn new(include_id: bool, username: String, email: String) -> Result<Self, Failure> {
        Self::validate_username(&username)?;
        Self::validate_email(&email)?;

        Ok(AccountEntity {
            base: BaseEntity::new(include_id),
            username,
            email,
            avatar: String::new(),
            email_verified: false,
            bio: "Please write your bio.".to_string(),
            is_active: true,
        })
    }

    fn validate_email(email: &str) -> Result<(), Failure> {
        let regex = Regex::new(EMAIL_REGEX)
            .map_err(|e| Failure::ValidationError(format!("Failed to compile email regex: {}", e)))?;

        if !regex.is_match(email) {
            return Err(Failure::ValidationError(format!("Please provide a valid email address: {}", email)));
        }

        Ok(())
    }

    fn validate_username(username: &str) -> Result<(), Failure> {
        if username.len() > MAX_USERNAME_LENGTH {
            return Err(Failure::ValidationError(format!(
                "Username must not exceed {} characters",
                MAX_USERNAME_LENGTH
            )));
        }

        Ok(())
    }
}
