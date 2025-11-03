use crate::apis::auth_api::AuthApi;
use crate::entities::account_entity::AccountEntity;
use crate::services::{
    account_service::AccountService, auth_service::AuthService, provider_service::ProviderService,
    session_service::SessionService,
};
use crate::usecases::auth_usecases::{AuthParams, AuthResponse, ManageSessionUseCases, RefreshTokenParams};
use async_trait::async_trait;
use shared::models::failure::Failure;
use std::sync::Arc;

pub struct AuthAppService {
    // services
    auth_service: Arc<dyn AuthService>,
    account_service: Arc<dyn AccountService>,
    provider_service: Arc<dyn ProviderService>,
    session_service: Arc<dyn SessionService>,

    // apis
    auth_api: Arc<dyn AuthApi>,
}

impl AuthAppService {
    pub fn new(
        auth_service: Arc<dyn AuthService>,
        account_service: Arc<dyn AccountService>,
        provider_service: Arc<dyn ProviderService>,
        session_service: Arc<dyn SessionService>,
        auth_api: Arc<dyn AuthApi>,
    ) -> Self {
        Self { auth_service, account_service, provider_service, session_service, auth_api }
    }
}

// region ============================== MANAGE SESSION USE CASES ==============================
#[async_trait]
impl ManageSessionUseCases for AuthAppService {
    async fn sign_up(&self, params: &AuthParams) -> Result<AccountEntity, Failure> {
        // 1. Check if email already exists
        if self.account_service.check_email_exists(&params.email).await? {
            return Err(Failure::Conflict("Email already exists".to_string()));
        }

        // 2. Hash password before saving
        let hashed_password = self.auth_service.hash_password(&params.password)?;

        // 3. Create account & provider
        let username = params.email.split("@").collect::<Vec<&str>>()[0].to_string();
        let account = self.account_service.create_account(username, params.email.clone()).await?;

        Ok(account)
    }

    async fn sign_in(&self, params: &AuthParams) -> Result<AuthResponse, Failure> {
        Ok(AuthResponse { access_token: "TOKEN".to_string(), refresh_token: "TOKEN".to_string() })
    }

    async fn refresh_token(&self, params: &RefreshTokenParams) -> Result<AuthResponse, Failure> {
        Ok(AuthResponse { access_token: "TOKEN".to_string(), refresh_token: "TOKEN".to_string() })
    }
}
// endregion ============================== MANAGE SESSION USE CASES ==============================
