use crate::entities::category_entity::CategoryEntity;
use async_trait::async_trait;
use serde::Deserialize;
use shared::types::DomainResponse;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryParams {
    #[validate(length(min = 1, max = 50, message = "Category name must be between 1 and 50 characters"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategoryParams {
    #[validate(length(min = 1, max = 50, message = "Category name must be between 1 and 50 characters"))]
    pub name: String,
}

#[async_trait]
pub trait ManageCategoryUseCase: Send + Sync {
    async fn create_category(&self, params: &CreateCategoryParams) -> DomainResponse<CategoryEntity>;
    async fn update_category(&self, category_id: &str, params: &UpdateCategoryParams)
    -> DomainResponse<CategoryEntity>;
    async fn delete_category(&self, category_id: &str) -> DomainResponse<CategoryEntity>;
    async fn find_category_by_id(&self, category_id: &str) -> DomainResponse<CategoryEntity>;
    async fn find_categories(&self) -> DomainResponse<Vec<CategoryEntity>>;
}
