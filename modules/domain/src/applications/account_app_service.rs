use crate::entities::account_entity::AccountEntity;
use crate::entities::experience_entity::ExperienceEntity;
use crate::services::account_service::AccountService;
use crate::services::experience_service::ExperienceService;
use crate::usecases::account_usecases::{
    AddExperienceToAccountParams, FindAccountsQuery, ManageAccountsUseCase, ManageExperienceAccountUseCase,
};
use async_trait::async_trait;
use shared::models::failure::Failure;
use shared::models::paginate::Paginate;
use shared::types::DomainResponse;
use std::sync::Arc;

pub struct AccountAppService {
    account_service: Arc<dyn AccountService>,
    experience_service: Arc<dyn ExperienceService>,
}

impl AccountAppService {
    pub fn new(account_service: Arc<dyn AccountService>, experience_service: Arc<dyn ExperienceService>) -> Self {
        Self { account_service, experience_service }
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
impl ManageExperienceAccountUseCase for AccountAppService {
    async fn find_experiences_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<ExperienceEntity>> {
        self.experience_service.find_by_account_id(account_id).await
    }

    async fn add_experience_to_account(
        &self,
        account_id: &str,
        params: &AddExperienceToAccountParams,
    ) -> DomainResponse<ExperienceEntity> {
        // 1. Verify account exists
        let account = self.account_service.find_account_by_id(account_id).await?;
        if account.is_none() {
            return Err(Failure::NotFound(format!("Account with id {} not found", account_id)));
        }

        // 2. Check for existing experience with same company
        let existing_experience = self.experience_service.find_by_company(&params.company).await?;
        if existing_experience.is_some() {
            return Err(Failure::Conflict(format!("Experience with company {} already exists", params.company)));
        }

        let experience = self
            .experience_service
            .create_experience(
                account_id,
                &params.technologies,
                &params.position,
                &params.responsibility,
                &params.company,
                &params.location,
                params.start_date,
                params.end_date,
                params.is_current,
            )
            .await?;

        Ok(experience)
    }
}

// endregion =================================== MANAGE WORKS ACCOUNT USE CASE ===================================
