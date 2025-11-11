use async_trait::async_trait;
use reqwest::Client;

// shared modules
use domain::apis::auth_api::AuthApi;
use shared::models::failure::Failure;
use shared::types::DomainResponse;

pub struct AuthApiImpl {
    client: Client,
}

impl AuthApiImpl {
    pub fn new() -> Self {
        AuthApiImpl { client: Client::new() }
    }
}

#[async_trait]
impl AuthApi for AuthApiImpl {
    async fn verify_google_token(&self, id_token: &str) -> DomainResponse<()> {
        // Verify token with Google
        let url = format!("https://oauth2.googleapis.com/tokeninfo?id_token={}", id_token);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| Failure::ExternalServiceError(format!("Failed to verify Google token: {}", e)))?;

        if !response.status().is_success() {
            return Err(Failure::Unauthorized("Invalid Google token".to_string()));
        }

        println!("Google Response: {:?}", response.text().await);
        Ok(())
    }
}
