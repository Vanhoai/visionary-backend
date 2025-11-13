use serde::{Deserialize, Serialize};

// shared modules
use shared::{models::failure::Failure, types::DomainResponse};

// internal modules
use crate::entities::base_entity::BaseEntity;

// Define rules for blog entity
static MAX_BLOG_MARKDOWN_SIZE: usize = 10 * 1024 * 1024; // 10 MB
static MAX_BLOG_NAME_SIZE: usize = 255;
static MAX_BLOG_DESCRIPTION_SIZE: usize = 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub author_id: String,
    pub categories: Vec<String>,
    pub name: String,
    pub description: String,
    pub is_published: bool,
    pub markdown: String,
    pub stars: i32,
    pub views: i32,
    pub estimated_read_time: i32,
}

impl BlogEntity {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        include_id: bool,
        author_id: &str,
        categories: &Vec<String>,
        name: &str,
        description: &str,
        is_published: bool,
        markdown: &str,
        estimated_read_time: i32,
    ) -> BlogEntity {
        BlogEntity {
            base: BaseEntity::new(include_id),
            author_id: author_id.to_string(),
            categories: categories.to_owned(),
            name: name.to_string(),
            description: description.to_string(),
            is_published,
            markdown: markdown.to_string(),
            stars: 0,
            views: 0,
            estimated_read_time,
        }
    }

    pub fn validate_markdown_size(markdown: &str) -> DomainResponse<()> {
        if markdown.len() > MAX_BLOG_MARKDOWN_SIZE {
            return Err(Failure::ValidationError(format!(
                "Blog markdown size exceeds the maximum limit of {} bytes",
                MAX_BLOG_MARKDOWN_SIZE
            )));
        }

        Ok(())
    }

    pub fn validate_name_size(name: &str) -> DomainResponse<()> {
        if name.len() > MAX_BLOG_NAME_SIZE {
            return Err(Failure::ValidationError(format!(
                "Blog name size exceeds the maximum limit of {} bytes",
                MAX_BLOG_NAME_SIZE
            )));
        }

        Ok(())
    }

    pub fn validate_description_size(description: &str) -> DomainResponse<()> {
        if description.len() > MAX_BLOG_DESCRIPTION_SIZE {
            return Err(Failure::ValidationError(format!(
                "Blog description size exceeds the maximum limit of {} bytes",
                MAX_BLOG_DESCRIPTION_SIZE
            )));
        }

        Ok(())
    }
}
