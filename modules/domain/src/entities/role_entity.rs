use serde::{Deserialize, Serialize};

// shared modules
use shared::models::failure::Failure;

// internal modules
use crate::entities::base_entity::BaseEntity;
use crate::values::roles::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub account_id: String,
    pub role_name: String,
}

impl RoleEntity {
    pub fn new(include_id: bool, account_id: String, role: Role) -> Result<Self, Failure> {
        Ok(RoleEntity { base: BaseEntity::new(include_id), account_id, role_name: role.as_str().to_string() })
    }
}
