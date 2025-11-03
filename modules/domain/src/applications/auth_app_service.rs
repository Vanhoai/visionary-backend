use crate::apis::auth_api::AuthApi;
use crate::entities::account_entity::AccountEntity;
use crate::services::{
    account_service::AccountService, auth_service::AuthService, provider_service::ProviderService,
    session_service::SessionService,
};
use crate::usecases::auth_usecases::{AuthParams, AuthResponse, ManageSessionUseCases, RefreshTokenParams};
use async_trait::async_trait;
use shared::jwt::service::JwtService;
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
        if self.account_service.check_email_exists(params.email.clone()).await? {
            return Err(Failure::Conflict("Email already exists".to_string()));
        }

        // 2. Hash password before saving
        let hashed_password = self.auth_service.hash_password(&params.password)?;

        // 3. Create and save account to database
        let username = params.email.split("@").collect::<Vec<&str>>()[0].to_string();
        let account_entity = self.account_service.create_account(username, params.email.clone()).await?;

        // 4. Create provider entry for the account with hashed password - AuthProvider
        let account_id = account_entity
            .base
            .id
            .clone()
            .ok_or(Failure::InternalServerError("Account ID should be present after creation".to_string()))?;

        self.provider_service.create_provider(account_id, "PASSWORD".to_string(), hashed_password).await?;
        Ok(account_entity)
    }

    async fn sign_in(&self, params: &AuthParams) -> Result<AuthResponse, Failure> {
        // 1. Retrieve account and provider
        let account_entity = self
            .account_service
            .find_by_email(params.email.clone())
            .await?
            .ok_or(Failure::NotFound("This email is not registered".to_string()))?;

        let account_id = account_entity
            .base
            .id
            .clone()
            .ok_or(Failure::InternalServerError("Account ID should be present".to_string()))?;

        let provider_entities = self.provider_service.find_by_account_id(&account_id).await?;

        // 2. Find password provider
        let password_provider = provider_entities
            .into_iter()
            .find(|p| p.auth_provider == "PASSWORD".to_string())
            .ok_or(Failure::Unauthorized("Password provider not found for this account".to_string()))?;

        // 3. Verify password
        if !self.auth_service.verify_password(&params.password, &password_provider.identify)? {
            return Err(Failure::Unauthorized("Invalid password".to_string()));
        }

        // 4. Generate tokens and create session
        let access_token = JwtService::generate_access_token(&account_id)
            .map_err(|e| Failure::InternalServerError(format!("Failed to generate access token: {}", e)))?;

        let refresh_token = JwtService::generate_refresh_token(&account_id)
            .map_err(|e| Failure::InternalServerError(format!("Failed to generate refresh token: {}", e)))?;

        Ok(AuthResponse { access_token, refresh_token })
    }

    async fn refresh_token(&self, params: &RefreshTokenParams) -> Result<AuthResponse, Failure> {
        Ok(AuthResponse { access_token: "TOKEN".to_string(), refresh_token: "TOKEN".to_string() })
    }
}
// endregion ============================== MANAGE SESSION USE CASES ==============================
