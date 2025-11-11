use serde::{Deserialize, Serialize};

// shared modules
use shared::{models::failure::Failure, types::DomainResponse};

// internal modules
use crate::entities::base_entity::BaseEntity;

static MAX_CATEGORY_NAME_LENGTH: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub name: String,
}

impl CategoryEntity {
    pub fn new(include_id: bool, name: &str) -> DomainResponse<Self> {
        Self::validate_name(&name)?;
        Ok(CategoryEntity { base: BaseEntity::new(include_id), name: name.to_string() })
    }

    pub fn validate_name(name: &str) -> DomainResponse<()> {
        if name.is_empty() || name.len() > MAX_CATEGORY_NAME_LENGTH {
            return Err(Failure::ValidationError(format!(
                "Category name must be between 1 and {} characters",
                MAX_CATEGORY_NAME_LENGTH
            )));
        }

        Ok(())
    }
}
