use async_trait::async_trait;
use std::sync::Arc;

// shared modules
use shared::{models::failure::Failure, types::DomainResponse};

// internal modules
use crate::{
    entities::category_entity::CategoryEntity,
    services::category_service::CategoryService,
    usecases::category_usecases::{CreateCategoryParams, ManageCategoryUseCase, UpdateCategoryParams},
};

pub struct CategoryAppService {
    category_service: Arc<dyn CategoryService>,
}

impl CategoryAppService {
    pub fn new(category_service: Arc<dyn CategoryService>) -> Self {
        Self { category_service }
    }
}

#[async_trait]
impl ManageCategoryUseCase for CategoryAppService {
    async fn create_category(&self, params: &CreateCategoryParams) -> DomainResponse<CategoryEntity> {
        // 1. Check if category with the same name already exists
        if self.category_service.check_category_with_name_exists(&params.name).await? {
            return Err(Failure::Conflict(format!("Category with name '{}' already exists", params.name)));
        }

        // 2. Create the new category
        let category = self.category_service.create_category(&params.name).await?;
        Ok(category)
    }

    async fn update_category(
        &self,
        category_id: &str,
        params: &UpdateCategoryParams,
    ) -> DomainResponse<CategoryEntity> {
        // 1. Find the existing category
        let mut category = self
            .category_service
            .find_category_by_id(category_id)
            .await?
            .ok_or_else(|| Failure::NotFound(format!("Category with id {} not found", category_id)))?;

        // 2. If the name is unchanged, return the existing category
        if category.name == params.name {
            return Ok(category);
        }

        // 3. Check if another category with the same name exists
        if self.category_service.check_category_with_name_exists(&params.name).await? {
            return Err(Failure::Conflict(format!("Category with name '{}' already exists", params.name)));
        }

        // 4. Update the category's name
        category.name = params.name.clone();
        let updated_category = self.category_service.update_category(category_id, &category).await?;
        Ok(updated_category)
    }

    async fn delete_category(&self, category_id: &str) -> DomainResponse<CategoryEntity> {
        self.category_service.find_and_delete_category(category_id).await
    }

    async fn find_category_by_id(&self, category_id: &str) -> DomainResponse<CategoryEntity> {
        let category = self
            .category_service
            .find_category_by_id(category_id)
            .await?
            .ok_or_else(|| Failure::NotFound(format!("Category with id {} not found", category_id)))?;

        Ok(category)
    }

    async fn find_categories(&self) -> DomainResponse<Vec<CategoryEntity>> {
        self.category_service.find_categories().await
    }
}
