use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AuthParams {
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenParams {
    pub refresh_token: String,
}

#[async_trait]
pub trait ManageSessionUseCases: Send + Sync {
    async fn sign_in(&self, params: &AuthParams) -> Result<AuthResponse, String>;
    async fn refresh_token(&self, params: &RefreshTokenParams) -> Result<AuthResponse, String>;
}
