use crate::entities::account_entity::AccountEntity;
use crate::entities::work_entity::WorkEntity;
use crate::services::account_service::AccountService;
use crate::usecases::account_usecases::{FindAccountsQuery, ManageAccountsUseCase, ManageWorksAccountUseCase};
use async_trait::async_trait;
use shared::models::paginate::Paginate;
use shared::types::DomainResponse;
use std::sync::Arc;
use crate::services::work_service::WorkService;

pub struct AccountAppService {
    account_service: Arc<dyn AccountService>,
    work_service: Arc<dyn WorkService>,
}

impl AccountAppService {
    pub fn new(account_service: Arc<dyn AccountService>, work_service: Arc<dyn WorkService>) -> Self {
        Self { account_service, work_service  }
    }
}

// region =================================== MANAGE ACCOUNT USE CASE ===================================
#[async_trait]
impl ManageAccountsUseCase for AccountAppService {
    async fn find_accounts(&self, query: &FindAccountsQuery) -> DomainResponse<(Paginate, Vec<AccountEntity>)> {
        self.account_service.find_accounts_paginated(query.paginate.page, query.paginate.page_size).await
    }
}
// endregion =================================== MANAGE ACCOUNT USE CASE ===================================

// region =================================== MANAGE WORKS ACCOUNT USE CASE ===================================
#[async_trait]
impl ManageWorksAccountUseCase for AccountAppService {
    async fn find_works_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<WorkEntity>> {
        todo!()
    }
}
// endregion =================================== MANAGE WORKS ACCOUNT USE CASE ===================================
