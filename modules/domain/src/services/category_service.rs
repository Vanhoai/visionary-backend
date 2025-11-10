use std::sync::Arc;

use crate::{entities::category_entity::CategoryEntity, repositories::category_repository::CategoryRepository};
use async_trait::async_trait;
use shared::{models::failure::Failure, types::DomainResponse};

#[async_trait]
pub trait CategoryService: Send + Sync {
    async fn find_category_by_id(&self, category_id: &str) -> DomainResponse<Option<CategoryEntity>>;
    async fn find_categories(&self) -> DomainResponse<Vec<CategoryEntity>>;
    async fn check_category_with_name_exists(&self, name: &str) -> DomainResponse<bool>;
    async fn create_category(&self, name: &str) -> DomainResponse<CategoryEntity>;
    async fn update_category(&self, category_id: &str, entity: &CategoryEntity) -> DomainResponse<CategoryEntity>;
    async fn find_and_delete_category(&self, category_id: &str) -> DomainResponse<CategoryEntity>;
}

pub struct CategoryServiceImpl {
    repository: Arc<dyn CategoryRepository>,
}

impl CategoryServiceImpl {
    pub fn new(repository: Arc<dyn CategoryRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl CategoryService for CategoryServiceImpl {
    async fn find_category_by_id(&self, category_id: &str) -> DomainResponse<Option<CategoryEntity>> {
        self.repository.find(category_id).await
    }

    async fn find_categories(&self) -> DomainResponse<Vec<CategoryEntity>> {
        self.repository.finds().await
    }

    async fn check_category_with_name_exists(&self, name: &str) -> DomainResponse<bool> {
        let category = self.repository.find_by_name(name).await?;
        Ok(category.is_some())
    }

    async fn create_category(&self, name: &str) -> DomainResponse<CategoryEntity> {
        let entity = CategoryEntity::new(false, name)?;
        self.repository.create(&entity).await
    }

    async fn update_category(&self, category_id: &str, entity: &CategoryEntity) -> DomainResponse<CategoryEntity> {
        self.repository.update(&category_id.to_string(), entity).await
    }

    async fn find_and_delete_category(&self, category_id: &str) -> DomainResponse<CategoryEntity> {
        // Hard delete the category
        self.repository.find_and_remove(category_id).await
    }
}
