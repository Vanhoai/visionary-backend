#[derive(Debug, Clone)]
pub struct SessionEntity {
    pub id: String,
    pub account_id: String,
    pub jit: String,
    pub expires_at: i64,
    pub ip_address: String,
    pub user_agent: String,
    pub device_type: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}