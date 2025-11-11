use serde::{Deserialize, Serialize};

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::entities::base_entity::BaseEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub author_id: String,
    pub category_id: String,
    pub name: String,
    pub description: String,
    pub is_published: bool,
    pub markdown: String,
    pub stars: i32,
    pub views: i32,
    pub estimate_read_time: i32,
}

impl BlogEntity {
    pub fn new(
        include_id: bool,
        author_id: String,
        category_id: String,
        name: String,
        description: String,
        is_published: bool,
        markdown: String,
        estimate_read_time: i32,
    ) -> DomainResponse<Self> {
        // add validation logic here if needed

        Ok(BlogEntity {
            base: BaseEntity::new(include_id),
            author_id,
            category_id,
            name,
            description,
            is_published,
            markdown,
            stars: 0,
            views: 0,
            estimate_read_time,
        })
    }
}
