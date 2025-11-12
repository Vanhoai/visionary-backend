use serde::{Deserialize, Serialize};

// shared modules
use domain::entities::category_entity::CategoryEntity;

// internal modules
use crate::secondary::repositories::{
    models::base_schema::{MongoBaseSchema, ScyllaBaseSchema},
    mongodb::mongo_base_repository,
    scylla::scylla_base_repository,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MongoCategorySchema {
    #[serde(flatten)]
    pub base: MongoBaseSchema,
    pub name: String,
}

impl mongo_base_repository::EntitySchema<CategoryEntity> for MongoCategorySchema {
    fn from_entity(entity: &CategoryEntity) -> Self {
        MongoCategorySchema { base: MongoBaseSchema::from_entity(&entity.base), name: entity.name.clone() }
    }

    fn to_entity(&self) -> CategoryEntity {
        CategoryEntity { base: self.base.to_entity(), name: self.name.clone() }
    }
}

#[derive(Debug, Clone)]
pub struct ScyllaCategorySchema {
    pub base: ScyllaBaseSchema,
    pub name: String,
}

impl scylla_base_repository::EntitySchema<CategoryEntity> for ScyllaCategorySchema {
    fn from_entity(entity: &CategoryEntity) -> Self {
        ScyllaCategorySchema { base: ScyllaBaseSchema::from_entity(&entity.base), name: entity.name.clone() }
    }

    fn to_entity(&self) -> CategoryEntity {
        CategoryEntity { base: self.base.to_entity(), name: self.name.clone() }
    }
}
