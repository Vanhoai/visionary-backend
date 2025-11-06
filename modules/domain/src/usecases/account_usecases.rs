use crate::entities::account_entity::AccountEntity;
use crate::entities::experience_entity::ExperienceEntity;
use async_trait::async_trait;
use serde::Deserialize;
use shared::{
    models::paginate::{BasePaginateQuery, Paginate},
    types::DomainResponse,
};
use validator::Validate;

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
