use crate::entities::base_entity::BaseEntity;
use crate::values::auth_provider::AuthProvider;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ProviderEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub account_id: String,
    pub auth_provider: String,
    pub identify: String,
}

impl ProviderEntity {
    pub fn new(include_id: bool, account_id: String, provider: AuthProvider, identify: String) -> Self {
        Self { base: BaseEntity::new(include_id), account_id, auth_provider: provider.to_string(), identify }
    }
}
