use async_trait::async_trait;
use std::sync::Arc;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::{entities::blog_entity::BlogEntity, repositories::blog_repository::BlogRepository};

#[async_trait]
pub trait BlogService: Send + Sync {
    #[allow(clippy::too_many_arguments)]
    async fn create_blog(
        &self,
        account_id: &str,
        name: &str,
        description: &str,
        markdown: &str,
        categories: &Vec<String>,
        is_published: bool,
        estimated_read_time: i32,
    ) -> DomainResponse<BlogEntity>;

    async fn update_blog(
        &self,
        account_id: &str,
        blog_id: &str,
        name: &str,
        description: &str,
        markdown: &str,
        categories: &Vec<String>,
        estimated_read_time: i32,
    ) -> DomainResponse<BlogEntity>;

    async fn delete_blog(&self, blog_id: &str) -> DomainResponse<bool>;
}

pub struct BlogServiceImpl {
    repository: Arc<dyn BlogRepository>,
}

impl BlogServiceImpl {
    pub fn new(repository: Arc<dyn BlogRepository>) -> Self {
        BlogServiceImpl { repository }
    }
}

#[async_trait]
impl BlogService for BlogServiceImpl {
    #[allow(clippy::too_many_arguments)]
    async fn create_blog(
        &self,
        author_id: &str,
        name: &str,
        description: &str,
        markdown: &str,
        categories: &Vec<String>,
        is_published: bool,
        estimated_read_time: i32,
    ) -> DomainResponse<BlogEntity> {
        BlogEntity::validate_name_size(name)?;
        BlogEntity::validate_description_size(description)?;
        BlogEntity::validate_markdown_size(markdown)?;

        let blog_entity = BlogEntity::new(
            false,
            author_id,
            categories,
            name,
            description,
            is_published,
            markdown,
            estimated_read_time,
        );

        self.repository.create(&blog_entity).await
    }

    async fn update_blog(
        &self,
        _account_id: &str,
        _blog_id: &str,
        _name: &str,
        _description: &str,
        _markdown: &str,
        _categories: &Vec<String>,
        _estimated_read_time: i32,
    ) -> DomainResponse<BlogEntity> {
        todo!()
    }

    async fn delete_blog(&self, _blog_id: &str) -> DomainResponse<bool> {
        todo!()
    }
}
