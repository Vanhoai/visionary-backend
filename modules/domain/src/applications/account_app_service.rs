use async_trait::async_trait;
use std::sync::Arc;

// shared modules
use shared::models::failure::Failure;
use shared::models::paginate::Paginate;
use shared::types::DomainResponse;

// internal modules
use crate::entities::{account_entity::AccountEntity, blog_entity::BlogEntity, role_entity::RoleEntity};
use crate::services::{account_service::AccountService, blog_service::BlogService, role_service::RoleService};
use crate::usecases::account_usecases::{
    AddRoleToAccountParams, FindAccountWithEmailQuery, FindAccountsQuery, ManageAccountsUseCase,
    ManageRoleAccountUseCase, MangeBlogAccountUseCase, PublishBlogParams, UpdateBlogParams, UpdateRoleToAccountParams,
};

pub struct AccountAppService {
    account_service: Arc<dyn AccountService>,
    role_service: Arc<dyn RoleService>,
    blog_service: Arc<dyn BlogService>,
}

impl AccountAppService {
    pub fn new(
        account_service: Arc<dyn AccountService>,
        role_service: Arc<dyn RoleService>,
        blog_service: Arc<dyn BlogService>,
    ) -> Self {
        Self { account_service, role_service, blog_service }
    }
}

// region =================================== MANAGE ACCOUNT USE CASE ===================================
#[async_trait]
impl ManageAccountsUseCase for AccountAppService {
    async fn find_accounts(&self, query: &FindAccountsQuery) -> DomainResponse<(Paginate, Vec<AccountEntity>)> {
        self.account_service.find_accounts_paginated(query.paginate.page, query.paginate.page_size).await
    }

    async fn find_account_with_id(&self, account_id: &str) -> DomainResponse<Option<AccountEntity>> {
        self.account_service.find_account_by_id(account_id).await
    }

    async fn find_account_with_email(
        &self,
        params: &FindAccountWithEmailQuery,
    ) -> DomainResponse<Option<AccountEntity>> {
        self.account_service.find_by_email(&params.email).await
    }
}
// endregion =================================== MANAGE ACCOUNT USE CASE ===================================

// region =================================== MANAGE ROLES ACCOUNT USE CASE ===================================
#[async_trait]
impl ManageRoleAccountUseCase for AccountAppService {
    async fn add_role_to_account(
        &self,
        account_id: &str,
        params: &AddRoleToAccountParams,
    ) -> DomainResponse<RoleEntity> {
        // 1. Verify account exists
        let account = self.account_service.find_account_by_id(account_id).await?;
        if account.is_none() {
            return Err(Failure::NotFound(format!("Account with id {} not found", account_id)));
        }

        // 2. Check for existing role
        let existing_role = self.role_service.find_role_by_account_id(account_id).await?;
        if existing_role.is_some() {
            return Err(Failure::Conflict(format!("Role for account id {} already exists", account_id)));
        }

        // 3. Create role
        Ok(self.role_service.create_role(account_id, &params.role_name).await?)
    }

    async fn update_role_for_account(
        &self,
        account_id: &str,
        params: &UpdateRoleToAccountParams,
    ) -> DomainResponse<RoleEntity> {
        // 1. Verify account exists
        let account = self.account_service.find_account_by_id(account_id).await?;
        if account.is_none() {
            return Err(Failure::NotFound(format!("Account with id {} not found", account_id)));
        }

        // 2. Find and update role
        let role_updated = self.role_service.find_and_update_role_by_account_id(account_id, &params.role_name).await?;
        Ok(role_updated)
    }

    async fn find_role_by_account_id(&self, account_id: &str) -> DomainResponse<RoleEntity> {
        // 1. Verify account exists
        let account = self.account_service.find_account_by_id(account_id).await?;
        if account.is_none() {
            return Err(Failure::NotFound(format!("Account with id {} not found", account_id)));
        }

        // 2. Find role
        let role = self.role_service.find_role_by_account_id(account_id).await?;
        if role.is_none() {
            return Err(Failure::NotFound(format!("Role for account id {} not found", account_id)));
        }

        Ok(role.unwrap())
    }
}
// endregion =================================== MANAGE ROLES ACCOUNT USE CASE ===================================

// region =================================== MANAGE BLOG ACCOUNT USE CASE ===================================
#[async_trait]
impl MangeBlogAccountUseCase for AccountAppService {
    async fn publish_account_blog(&self, account_id: &str, params: &PublishBlogParams) -> DomainResponse<BlogEntity> {
        // 1. Verify account exists
        self.account_service
            .find_account_by_id(account_id)
            .await?
            .ok_or(Failure::NotFound(format!("Account with id {} not found", account_id)))?;

        // 2. Publish blog
        let blog_entity = self
            .blog_service
            .create_blog(
                account_id,
                &params.name,
                &params.description,
                &params.markdown,
                &params.categories,
                params.is_published,
                params.estimated_read_time,
            )
            .await?;

        Ok(blog_entity)
    }

    async fn update_account_blog(
        &self,
        _account_id: &str,
        _blog_id: &str,
        _params: &UpdateBlogParams,
    ) -> DomainResponse<BlogEntity> {
        // Implementation goes here
        unimplemented!()
    }

    async fn delete_account_blog(&self, _account_id: &str, _blog_id: &str) -> DomainResponse<()> {
        // Implementation goes here
        unimplemented!()
    }
}
// endregion =================================== MANAGE BLOG ACCOUNT USE CASE ===================================
