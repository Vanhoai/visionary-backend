use mongodb::bson::oid::ObjectId;
use scylla::SerializeRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// shared modules
use domain::entities::base_entity::BaseEntity;
use domain::entities::blog_entity::BlogEntity;

// internal modules
use crate::secondary::repositories::models::base_schema::MongoBaseSchema;
use crate::secondary::repositories::mongodb::mongo_base_repository;
use crate::secondary::repositories::scylla::scylla_base_repository;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoBlogSchema {
    #[serde(flatten)]
    pub base: MongoBaseSchema,
    pub author_id: ObjectId,
    pub category_id: ObjectId,
    pub name: String,
    pub description: String,
    pub is_published: bool,
    pub markdown: String,
    pub stars: i32,
    pub views: i32,
    pub estimated_read_time: i32,
}

impl mongo_base_repository::EntitySchema<BlogEntity> for MongoBlogSchema {
    fn from_entity(entity: &BlogEntity) -> Self {
        MongoBlogSchema {
            base: MongoBaseSchema::from_entity(&entity.base),
            author_id: ObjectId::parse_str(&entity.author_id).unwrap(),
            category_id: ObjectId::parse_str(&entity.category_id).unwrap(),
            name: entity.name.clone(),
            description: entity.description.clone(),
            is_published: entity.is_published,
            markdown: entity.markdown.clone(),
            stars: entity.stars,
            views: entity.views,
            estimated_read_time: entity.estimated_read_time,
        }
    }

    fn to_entity(&self) -> BlogEntity {
        BlogEntity {
            base: self.base.to_entity(),
            author_id: self.author_id.to_hex(),
            category_id: self.category_id.to_hex(),
            name: self.name.clone(),
            description: self.description.clone(),
            is_published: self.is_published,
            markdown: self.markdown.clone(),
            stars: self.stars,
            views: self.views,
            estimated_read_time: self.estimated_read_time,
        }
    }
}

#[derive(Debug, Clone, SerializeRow)]
pub struct ScyllaBlogSchema {
    pub id: Option<Uuid>,
    pub author_id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub description: String,
    pub is_published: bool,
    pub markdown: String,
    pub stars: i32,
    pub views: i32,
    pub estimated_read_time: i32,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl scylla_base_repository::EntitySchema<BlogEntity> for ScyllaBlogSchema {
    fn from_entity(entity: &BlogEntity) -> Self {
        ScyllaBlogSchema {
            id: entity.base.id.as_ref().and_then(|id| Uuid::parse_str(id).ok()),
            created_at: entity.base.created_at,
            updated_at: entity.base.updated_at,
            deleted_at: entity.base.deleted_at,
            author_id: Uuid::parse_str(&entity.author_id).unwrap(),
            category_id: Uuid::parse_str(&entity.category_id).unwrap(),
            name: entity.name.clone(),
            description: entity.description.clone(),
            is_published: entity.is_published,
            markdown: entity.markdown.clone(),
            stars: entity.stars,
            views: entity.views,
            estimated_read_time: entity.estimated_read_time,
        }
    }

    fn to_entity(&self) -> BlogEntity {
        BlogEntity {
            base: BaseEntity {
                id: self.id.map(|id| id.to_string()),
                created_at: self.created_at,
                updated_at: self.updated_at,
                deleted_at: self.deleted_at,
            },
            author_id: self.author_id.to_string(),
            category_id: self.category_id.to_string(),
            name: self.name.clone(),
            description: self.description.clone(),
            is_published: self.is_published,
            markdown: self.markdown.clone(),
            stars: self.stars,
            views: self.views,
            estimated_read_time: self.estimated_read_time,
        }
    }

    fn columns() -> &'static str {
        "id, author_id, category_id, name, description, is_published, markdown, stars, views, estimated_read_time, created_at, updated_at, deleted_at"
    }

    fn insert_placeholders() -> &'static str {
        "?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?"
    }
}
