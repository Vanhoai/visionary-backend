#[derive(Clone, Debug)]
pub struct ProviderEntity {
    pub id: String,
    pub account_id: String,
    pub provider: String,
    pub identify: String,
    pub extra: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}