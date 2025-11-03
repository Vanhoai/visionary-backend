use crate::entities::base_entity::BaseEntity;
use crate::values::auth_provider::AuthProvider;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ProviderEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub account_id: String,
    pub auth_provider: String,
    pub identify: String,
    pub extra: Option<String>,
}

impl ProviderEntity {
    pub fn new(account_id: String, provider: AuthProvider, identify: String, extra: Option<String>) -> Self {
        Self { base: BaseEntity::new(), account_id, auth_provider: provider.to_string(), identify, extra }
    }
}
