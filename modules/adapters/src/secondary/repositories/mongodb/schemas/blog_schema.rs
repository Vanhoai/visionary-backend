use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// shared modules
use domain::entities::blog_entity::BlogEntity;

// internal modules
use crate::secondary::repositories::mongodb::mongo_base_repository::EntitySchema;
use crate::secondary::repositories::mongodb::schemas::base_schema::BaseSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogSchema {
    #[serde(flatten)]
    base: BaseSchema,
    pub author_id: ObjectId,
    pub category_id: ObjectId,
    pub name: String,
    pub description: String,
    pub is_published: bool,
    pub markdown: String,
    pub stars: i32,
    pub views: i32,
    pub estimate_read_time: i32,
}

impl EntitySchema<BlogEntity> for BlogSchema {
    fn from_entity(entity: BlogEntity) -> Self {
        BlogSchema {
            base: BaseSchema::from_entity(entity.base),
            author_id: ObjectId::parse_str(&entity.author_id).unwrap(),
            category_id: ObjectId::parse_str(&entity.category_id).unwrap(),
            name: entity.name,
            description: entity.description,
            is_published: entity.is_published,
            markdown: entity.markdown,
            stars: entity.stars,
            views: entity.views,
            estimate_read_time: entity.estimate_read_time,
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
            estimate_read_time: self.estimate_read_time,
        }
    }
}
