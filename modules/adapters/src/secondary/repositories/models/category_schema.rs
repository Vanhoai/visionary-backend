use scylla::SerializeRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// shared modules
use domain::entities::{base_entity::BaseEntity, category_entity::CategoryEntity};

// internal modules
use crate::secondary::repositories::{
    models::base_schema::MongoBaseSchema, mongodb::mongo_base_repository, scylla::scylla_base_repository,
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

#[derive(Debug, Clone, SerializeRow)]
pub struct ScyllaCategorySchema {
    pub id: Option<Uuid>,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl scylla_base_repository::EntitySchema<CategoryEntity> for ScyllaCategorySchema {
    fn from_entity(entity: &CategoryEntity) -> Self {
        ScyllaCategorySchema {
            id: entity.base.id.as_ref().and_then(|id| Uuid::parse_str(id).ok()),
            name: entity.name.clone(),
            created_at: entity.base.created_at,
            updated_at: entity.base.updated_at,
            deleted_at: entity.base.deleted_at,
        }
    }

    fn to_entity(&self) -> CategoryEntity {
        CategoryEntity {
            base: BaseEntity {
                id: self.id.map(|id| id.to_string()),
                created_at: self.created_at,
                updated_at: self.updated_at,
                deleted_at: self.deleted_at,
            },
            name: self.name.clone(),
        }
    }

    fn columns() -> &'static str {
        "id, name, created_at, updated_at, deleted_at"
    }

    fn insert_placeholders() -> &'static str {
        "?, ?, ?, ?, ?"
    }
}
