use crate::entities::base_entity::BaseEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub account_id: String,
    pub jit: String,
    pub refresh_token: String,
    pub expires_at: i64,
    pub ip_address: String,
    pub user_agent: String,
    pub device_type: String,
}

impl SessionEntity {
    pub fn new(
        include_id: bool,
        account_id: String,
        jit: String,
        refresh_token: String,
        expires_at: i64,
        ip_address: String,
        user_agent: String,
        device_type: String,
    ) -> Self {
        Self {
            base: BaseEntity::new(include_id),
            account_id,
            jit,
            refresh_token,
            expires_at,
            ip_address,
            user_agent,
            device_type,
        }
    }
}
