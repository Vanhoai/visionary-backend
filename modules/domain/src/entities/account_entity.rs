use chrono::format::Numeric::Timestamp;
use serde::Serialize;
use shared::models::failure::Failure;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct AccountEntity {
    pub id: String,
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

impl AccountEntity {
    const MAX_USERNAME_LENGTH: usize = 20;

    pub fn new(username: String, email: String) -> Result<Self, Failure> {
        Self::validate_username(&username)?;

        let uuid = Uuid::now_v7().to_string();
        let now = chrono::Utc::now().timestamp();

        Ok(AccountEntity {
            id: uuid,
            username,
            avatar: String::new(),
            email,
            email_verified: false,
            bio: String::new(),
            is_active: true,
            created_at: now,
            updated_at: now,
            deleted_at: None,
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
