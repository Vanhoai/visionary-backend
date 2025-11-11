use serde::{Deserialize, Serialize};

// shared modules
use domain::entities::category_entity::CategoryEntity;

// internal modules
use crate::secondary::repositories::mongodb::{mongo_base_repository::EntitySchema, schemas::base_schema::BaseSchema};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategorySchema {
    #[serde(flatten)]
    pub base: BaseSchema,
    pub name: String,
}

impl EntitySchema<CategoryEntity> for CategorySchema {
    fn from_entity(entity: CategoryEntity) -> Self {
        CategorySchema { base: BaseSchema::from_entity(entity.base), name: entity.name }
    }

    fn to_entity(&self) -> CategoryEntity {
        CategoryEntity { base: self.base.to_entity(), name: self.name.clone() }
    }
}
