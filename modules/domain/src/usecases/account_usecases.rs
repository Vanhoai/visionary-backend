use async_trait::async_trait;
use serde::Deserialize;
use validator::Validate;

// shared modules
use shared::{
    models::paginate::{BasePaginateQuery, Paginate},
    types::DomainResponse,
};

// internal modules
use crate::entities::account_entity::AccountEntity;
use crate::entities::experience_entity::ExperienceEntity;
use crate::entities::role_entity::RoleEntity;

// region =================================== MANAGE ACCOUNT USE CASE ===================================
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct FindAccountsQuery {
    #[serde(flatten)]
    pub paginate: BasePaginateQuery,
}

#[async_trait]
pub trait ManageAccountsUseCase: Send + Sync {
    async fn find_accounts(&self, query: &FindAccountsQuery) -> DomainResponse<(Paginate, Vec<AccountEntity>)>;
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

// region =================================== MANAGE WORKS ACCOUNT USE CASE ===================================

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AddExperienceToAccountParams {
    #[validate(length(min = 1, message = "At least one technology must be provided"))]
    pub technologies: Vec<String>,

    #[validate(length(min = 1, message = "Position must not be empty"))]
    pub position: String,

    #[validate(length(min = 1, message = "At least one responsibility must be provided"))]
    pub responsibility: Vec<String>,

    #[validate(length(min = 1, message = "Company must not be empty"))]
    pub company: String,

    #[validate(length(min = 1, message = "Location must not be empty"))]
    pub location: String,

    pub start_date: i64,
    pub end_date: Option<i64>,
    pub is_current: bool,
}

#[async_trait]
pub trait ManageExperienceAccountUseCase: Send + Sync {
    async fn find_experiences_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<ExperienceEntity>>;
    async fn add_experience_to_account(
        &self,
        account_id: &str,
        params: &AddExperienceToAccountParams,
    ) -> DomainResponse<ExperienceEntity>;
}
// endregion =================================== MANAGE WORKS ACCOUNT USE CASE ===================================
