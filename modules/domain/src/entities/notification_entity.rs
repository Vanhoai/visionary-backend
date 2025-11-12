use serde::{Deserialize, Serialize};

// internal modules
use crate::entities::base_entity::BaseEntity;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub account_id: String,
    pub message: String,
    pub is_read: bool,
}

impl NotificationEntity {
    pub fn new(include_id: bool, account_id: String, message: String, is_read: bool) -> Self {
        Self { base: BaseEntity::new(include_id), account_id, message, is_read }
    }
}
