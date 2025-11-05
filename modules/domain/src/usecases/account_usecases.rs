use crate::entities::account_entity::AccountEntity;
use crate::entities::work_entity::WorkEntity;
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
#[async_trait]
pub trait ManageWorksAccountUseCase: Send + Sync {
    async fn find_works_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<WorkEntity>>;
}
// endregion =================================== MANAGE WORKS ACCOUNT USE CASE ===================================
