use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use validator::Validate;

// shared modules
use shared::models::failure::Failure;
use shared::types::DomainResponse;

// internal modules
use crate::entities::account_entity::AccountEntity;

#[derive(Debug, Clone)]
pub struct SessionMetadata {
    pub ip_address: String,
    pub user_agent: String,
    pub device_type: String,
}

// region ================================= MANAGE SESSION AUTH USE CASES =================================
#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthParams {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenParams {
    #[validate(length(min = 1, message = "Make sure to provide a valid refresh token"))]
    pub refresh_token: String,
}

#[async_trait]
pub trait ManageSessionAuthUseCase: Send + Sync {
    async fn sign_up(&self, params: &AuthParams) -> Result<AccountEntity, Failure>;
    async fn sign_in(&self, params: &AuthParams, metadata: &SessionMetadata) -> Result<AuthResponse, Failure>;
    async fn refresh_token(
        &self,
        params: &RefreshTokenParams,
        metadata: &SessionMetadata,
    ) -> Result<AuthResponse, Failure>;
}
// endregion ================================= MANAGE SESSION AUTH USE CASES =================================

// region ================================= OAUTH2 AUTH USE CASES =================================
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2InitParams {
    pub provider: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2InitResponse {
    pub authorization_url: String,
    pub state: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct OAuth2CallbackParams {
    #[validate(length(min = 1, message = "Code cannot be empty"))]
    pub code: String,

    #[validate(length(min = 1, message = "State cannot be empty"))]
    pub state: String,
}

#[async_trait]
pub trait OAuth2UseCase: Send + Sync {
    async fn oauth2_init(&self, params: &OAuth2InitParams) -> DomainResponse<OAuth2InitResponse>;
    async fn oauth2_callback(
        &self,
        params: &OAuth2CallbackParams,
        metadata: &SessionMetadata,
    ) -> DomainResponse<AuthResponse>;
}
// endregion ================================= OAUTH2 AUTH USE CASES =================================
