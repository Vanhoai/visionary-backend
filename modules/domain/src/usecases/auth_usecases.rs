use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use validator::Validate;

use shared::models::failure::Failure;

use crate::entities::account_entity::AccountEntity;

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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenParams {
    pub refresh_token: String,
}

#[async_trait]
pub trait ManageSessionUseCases: Send + Sync {
    async fn sign_up(&self, params: &AuthParams) -> Result<AccountEntity, Failure>;
    async fn sign_in(&self, params: &AuthParams) -> Result<AuthResponse, Failure>;
    async fn refresh_token(&self, params: &RefreshTokenParams) -> Result<AuthResponse, Failure>;
}
