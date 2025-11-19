use async_trait::async_trait;
use serde::Deserialize;
use validator::Validate;

// shared modules
use shared::{
    models::paginate::{BasePaginateQuery, Paginate},
    types::DomainResponse,
};

// internal modules
use crate::entities::role_entity::RoleEntity;
use crate::entities::{account_entity::AccountEntity, blog_entity::BlogEntity};

// region =================================== MANAGE ACCOUNT USE CASE ===================================
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FindAccountsQuery {
    #[serde(flatten)]
    pub paginate: BasePaginateQuery,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FindAccountWithEmailQuery {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[async_trait]
pub trait ManageAccountsUseCase: Send + Sync {
    async fn find_accounts(&self, query: &FindAccountsQuery) -> DomainResponse<(Paginate, Vec<AccountEntity>)>;
    async fn find_account_with_id(&self, account_id: &str) -> DomainResponse<Option<AccountEntity>>;
    async fn find_account_with_email(&self, query: &FindAccountWithEmailQuery)
    -> DomainResponse<Option<AccountEntity>>;
}
// endregion =================================== MANAGE ACCOUNT USE CASE ===================================

// region =================================== MANAGE ROLE ACCOUNT USE CASE ===================================
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddRoleToAccountParams {
    #[validate(length(min = 1, message = "Role name must not be empty"))]
    pub role_name: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleToAccountParams {
    #[validate(length(min = 1, message = "Role name must not be empty"))]
    pub role_name: String,
}

#[async_trait]
pub trait ManageRoleAccountUseCase: Send + Sync {
    async fn add_role_to_account(
        &self,
        account_id: &str,
        params: &AddRoleToAccountParams,
    ) -> DomainResponse<RoleEntity>;

    async fn update_role_for_account(
        &self,
        account_id: &str,
        params: &UpdateRoleToAccountParams,
    ) -> DomainResponse<RoleEntity>;

    async fn find_role_by_account_id(&self, account_id: &str) -> DomainResponse<RoleEntity>;
}
// endregion =================================== MANAGE ROLE ACCOUNT USE CASE ===================================

// region =================================== MANAGE BLOG ACCOUNT USE CASE ===================================
#[derive(Debug, Deserialize, Validate)]
pub struct PublishBlogParams {
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,

    #[validate(length(min = 1, message = "Description must not be empty"))]
    pub description: String,

    #[validate(length(min = 1, message = "Markdown content must not be empty"))]
    pub markdown: String,

    #[validate(length(min = 1, message = "At least one category must be provided"))]
    pub categories: Vec<String>,

    #[validate(range(min = 1, message = "Estimated read time must be greater than zero"))]
    pub estimated_read_time: i32,
    pub is_published: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBlogParams {}

#[async_trait]
pub trait MangeBlogAccountUseCase: Send + Sync {
    async fn publish_account_blog(&self, account_id: &str, params: &PublishBlogParams) -> DomainResponse<BlogEntity>;

    async fn update_account_blog(
        &self,
        account_id: &str,
        blog_id: &str,
        params: &UpdateBlogParams,
    ) -> DomainResponse<BlogEntity>;

    async fn delete_account_blog(&self, account_id: &str, blog_id: &str) -> DomainResponse<()>;
}
// endregion =================================== MANAGE BLOG ACCOUNT USE CASE ===================================
