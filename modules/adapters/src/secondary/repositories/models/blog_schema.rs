use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// shared modules
use domain::entities::blog_entity::BlogEntity;
use uuid::Uuid;

// internal modules
use crate::secondary::repositories::models::base_schema::{MongoBaseSchema, ScyllaBaseSchema};
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

#[derive(Debug, Clone)]
pub struct ScyllaBlogSchema {
    pub base: ScyllaBaseSchema,
    pub author_id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub description: String,
    pub is_published: bool,
    pub markdown: String,
    pub stars: i32,
    pub views: i32,
    pub estimated_read_time: i32,
}

impl scylla_base_repository::EntitySchema<BlogEntity> for ScyllaBlogSchema {
    fn from_entity(entity: &BlogEntity) -> Self {
        ScyllaBlogSchema {
            base: ScyllaBaseSchema::from_entity(&entity.base),
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
            base: self.base.to_entity(),
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
}
