use scylla::SerializeRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// shared modules
use domain::entities::{base_entity::BaseEntity, project_entity::ProjectEntity};

// internal modules
use crate::secondary::repositories::{
    models::base_schema::MongoBaseSchema, mongodb::mongo_base_repository, scylla::scylla_base_repository,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoProjectSchema {
    #[serde(flatten)]
    base: MongoBaseSchema,
    pub cover: String,
    pub name: String,
    pub description: String,
    pub link: String,
    pub github: String,
    pub tags: Vec<String>,
    pub markdown: String,
}

impl mongo_base_repository::EntitySchema<ProjectEntity> for MongoProjectSchema {
    fn from_entity(entity: &ProjectEntity) -> Self {
        MongoProjectSchema {
            base: MongoBaseSchema::from_entity(&entity.base),
            cover: entity.cover.clone(),
            name: entity.name.clone(),
            description: entity.description.clone(),
            link: entity.link.clone(),
            github: entity.github.clone(),
            tags: entity.tags.to_owned(),
            markdown: entity.markdown.clone(),
        }
    }

    fn to_entity(&self) -> ProjectEntity {
        ProjectEntity {
            base: self.base.to_entity(),
            cover: self.cover.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            link: self.link.clone(),
            github: self.github.clone(),
            tags: self.tags.clone(),
            markdown: self.markdown.clone(),
        }
    }
}

#[derive(Debug, Clone, SerializeRow)]
pub struct ScyllaProjectSchema {
    pub id: Option<Uuid>,
    pub cover: String,
    pub name: String,
    pub description: String,
    pub link: String,
    pub github: String,
    pub tags: Vec<String>,
    pub markdown: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl scylla_base_repository::EntitySchema<ProjectEntity> for ScyllaProjectSchema {
    fn from_entity(entity: &ProjectEntity) -> Self {
        ScyllaProjectSchema {
            id: entity.base.id.as_ref().and_then(|id| Uuid::parse_str(id).ok()),
            created_at: entity.base.created_at,
            updated_at: entity.base.updated_at,
            deleted_at: entity.base.deleted_at,
            cover: entity.cover.clone(),
            name: entity.name.clone(),
            description: entity.description.clone(),
            link: entity.link.clone(),
            github: entity.github.clone(),
            tags: entity.tags.to_owned(),
            markdown: entity.markdown.clone(),
        }
    }

    fn to_entity(&self) -> ProjectEntity {
        ProjectEntity {
            base: BaseEntity {
                id: self.id.as_ref().map(|id| id.to_string()),
                created_at: self.created_at,
                updated_at: self.updated_at,
                deleted_at: self.deleted_at,
            },
            cover: self.cover.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            link: self.link.clone(),
            github: self.github.clone(),
            tags: self.tags.clone(),
            markdown: self.markdown.clone(),
        }
    }

    fn columns() -> &'static str {
        "id, cover, name, description, link, github, tags, markdown, created_at, updated_at, deleted_at"
    }

    fn insert_placeholders() -> &'static str {
        "?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?"
    }
}
