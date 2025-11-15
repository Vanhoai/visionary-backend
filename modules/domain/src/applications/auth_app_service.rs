use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

// shared modules
use shared::configs::APP_CONFIG;
use shared::jwt::service::JwtService;
use shared::models::failure::Failure;
use shared::oauth2::oauth2_providers::OAuth2Provider;
use shared::oauth2::oauth2_service::OAuth2Service;
use shared::types::DomainResponse;

// internal modules
use crate::apis::auth_api::AuthApi;
use crate::entities::account_entity::AccountEntity;
use crate::services::account_service::AccountService;
use crate::services::auth_service::AuthService;
use crate::services::provider_service::ProviderService;
use crate::services::role_service::RoleService;
use crate::services::session_service::SessionService;
use crate::usecases::auth_usecases::{
    AuthParams, AuthResponse, ManageSessionAuthUseCase, OAuth2CallbackParams, OAuth2InitParams, OAuth2InitResponse,
    OAuth2UseCase, RefreshTokenParams, SessionMetadata,
};

#[allow(dead_code)]
pub struct AuthAppService {
    // services
    auth_service: Arc<dyn AuthService>,
    account_service: Arc<dyn AccountService>,
    provider_service: Arc<dyn ProviderService>,
    session_service: Arc<dyn SessionService>,
    role_service: Arc<dyn RoleService>,

    // apis
    auth_api: Arc<dyn AuthApi>,
}

impl AuthAppService {
    pub fn new(
        auth_service: Arc<dyn AuthService>,
        account_service: Arc<dyn AccountService>,
        provider_service: Arc<dyn ProviderService>,
        session_service: Arc<dyn SessionService>,
        role_service: Arc<dyn RoleService>,
        auth_api: Arc<dyn AuthApi>,
    ) -> Self {
        Self { auth_service, account_service, provider_service, session_service, role_service, auth_api }
    }

    async fn generate_tokens_and_create_session(
        &self,
        account_id: &str,
        role: Option<String>,
        metadata: &SessionMetadata,
    ) -> Result<AuthResponse, Failure> {
        let jti = Uuid::now_v7().to_string();
        let access_token = JwtService::generate_access_token(account_id, &jti, role.clone())?;
        let refresh_token = JwtService::generate_refresh_token(account_id, &jti, role)?;

        // Calculate session expiry (same as refresh token expiry)
        let expires_at = (Utc::now().timestamp()) + APP_CONFIG.jwt.refresh_token_expiry;

        // Clean old sessions and create a new session with metadata
        self.session_service.clean_session_by_account_id(account_id).await?;
        self.session_service
            .create_session(
                account_id,
                &jti,
                expires_at,
                &metadata.ip_address,
                &metadata.user_agent,
                &metadata.device_type,
            )
            .await?;

        Ok(AuthResponse { access_token, refresh_token })
    }
}

// region ============================== MANAGE SESSION USE CASES ==============================
#[async_trait]
impl ManageSessionAuthUseCase for AuthAppService {
    async fn sign_up(&self, params: &AuthParams) -> Result<AccountEntity, Failure> {
        // 1. Check if email already exists
        if self.account_service.check_email_exists(&params.email).await? {
            return Err(Failure::Conflict("Email already exists".to_string()));
        }

        // 2. Hash password before saving
        let hashed_password = self.auth_service.hash_password(&params.password)?;

        // 3. Create and save account to database
        let username = params.email.split("@").collect::<Vec<&str>>()[0].to_string();
        let account_entity = self.account_service.create_account(&username, &params.email).await?;

        // 4. Create provider entry for the account with hashed password - AuthProvider
        let account_id = account_entity
            .base
            .id
            .clone()
            .ok_or(Failure::InternalServerError("Account ID should be present after creation".to_string()))?;

        self.provider_service.create_provider(&account_id, "PASSWORD", &hashed_password).await?;
        Ok(account_entity)
    }

    async fn sign_in(&self, params: &AuthParams, metadata: &SessionMetadata) -> Result<AuthResponse, Failure> {
        // 1. Retrieve account by email
        let account_entity = self
            .account_service
            .find_by_email(&params.email)
            .await?
            .ok_or(Failure::NotFound("This email is not registered".to_string()))?;

        let account_id = account_entity
            .base
            .id
            .clone()
            .ok_or(Failure::InternalServerError("Account ID should be present".to_string()))?;

        // 2. Retrieve account's providers and roles concurrently
        let (provider_entities, role_entity) = tokio::try_join!(
            self.provider_service.find_by_account_id(&account_id),
            self.role_service.find_role_by_account_id(&account_id)
        )?;

        let password_provider = provider_entities
            .into_iter()
            .find(|provider| provider.auth_provider == "PASSWORD")
            .ok_or(Failure::Unauthorized("Password provider not found for this account".to_string()))?;

        // 3. Verify password
        if !self.auth_service.verify_password(&params.password, &password_provider.identify)? {
            return Err(Failure::Unauthorized("Invalid password".to_string()));
        }

        // 4. Generate tokens and create session
        let role = if let Some(role_entity) = role_entity { Some(role_entity.role_name) } else { None };
        self.generate_tokens_and_create_session(&account_id, role, metadata).await
    }

    async fn refresh_token(
        &self,
        params: &RefreshTokenParams,
        metadata: &SessionMetadata,
    ) -> Result<AuthResponse, Failure> {
        // 1. Verify refresh token
        let token_data = JwtService::verify_access_token(&params.refresh_token)?;
        let account_id = token_data.claims.sub;
        let jti = token_data.claims.jti;
        let role = token_data.claims.role;

        // 2. Verify session exists
        let session_entity = self
            .session_service
            .find_by_jti(&jti)
            .await?
            .ok_or(Failure::Unauthorized("Session not found for the provided refresh token".to_string()))?;

        if session_entity.account_id != account_id || session_entity.jti != jti {
            return Err(Failure::Unauthorized("Session does not belong to the account".to_string()));
        }

        // 3. Generate new tokens
        let jti = Uuid::now_v7().to_string();
        let access_token = JwtService::generate_access_token(&account_id, &jti, role.clone())?;
        let refresh_token = JwtService::generate_refresh_token(&account_id, &jti, role)?;

        // 4. Calculate new session expiry
        let expires_at = (Utc::now().timestamp()) + APP_CONFIG.jwt.refresh_token_expiry;

        // 5. Invalidate old session & create new session
        self.session_service
            .create_session(
                &account_id,
                &jti,
                expires_at,
                &metadata.ip_address,
                &metadata.user_agent,
                &metadata.device_type,
            )
            .await?;

        self.session_service.invalidate_session(&session_entity.base.id.unwrap()).await?;
        Ok(AuthResponse { access_token, refresh_token })
    }
}
// endregion ============================== MANAGE SESSION USE CASES ==============================

// region ============================== OAUTH2 USE CASES ==============================
#[async_trait]
impl OAuth2UseCase for AuthAppService {
    async fn oauth2_init(&self, params: &OAuth2InitParams) -> DomainResponse<OAuth2InitResponse> {
        let oauth_provider = OAuth2Provider::from_string(&params.provider)?;
        match oauth_provider {
            OAuth2Provider::Google => {
                let (authorization_url, state) = OAuth2Service::get_google_auth_url();
                Ok(OAuth2InitResponse { authorization_url, state })
            },

            OAuth2Provider::GitHub => {
                let (authorization_url, state) = OAuth2Service::get_github_auth_url();
                Ok(OAuth2InitResponse { authorization_url, state })
            },
        }
    }

    async fn oauth2_callback(
        &self,
        params: &OAuth2CallbackParams,
        metadata: &SessionMetadata,
    ) -> DomainResponse<AuthResponse> {
        // 1. Exchange code for access token
        let access_token = OAuth2Service::exchange_google_code(&params.code, &params.state).await?;

        // 2. Get account information from Google
        let google_account = OAuth2Service::get_google_account_information(&access_token).await?;

        // 3. Check account existence
        let account_option = self.account_service.find_by_email(&google_account.email).await?;
        let account_id = match account_option {
            None => {
                let new_account =
                    self.account_service.create_account(&google_account.name, &google_account.email).await?;
                let new_account_id =
                    new_account.base.id.ok_or(Failure::InternalServerError("Failed to create account".to_string()))?;

                // Link Google provider
                self.provider_service.create_provider(&new_account_id, "GOOGLE", &google_account.id).await?;
                new_account_id
            },
            Some(account_entity) => account_entity
                .base
                .id
                .ok_or(Failure::InternalServerError("Account ID should be present".to_string()))?,
        };

        // 4. Retrieve roles (if any)
        let role_entity = self.role_service.find_role_by_account_id(&account_id).await?;
        let role = if let Some(role_entity) = role_entity { Some(role_entity.role_name) } else { None };

        // 5. Generate tokens and create session
        self.generate_tokens_and_create_session(&account_id, role, metadata).await
    }
}
// endregion ============================== OAUTH2 USE CASES ==============================
