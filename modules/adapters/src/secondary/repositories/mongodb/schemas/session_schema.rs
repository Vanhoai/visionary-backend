use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionSchema {
    pub _id: ObjectId,
    pub account_id: ObjectId,
    pub jit: String,
    pub expires_at: i64,
    pub ip_address: String,
    pub user_agent: String,
    pub device_type: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}
