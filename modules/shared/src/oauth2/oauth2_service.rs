use crate::models::failure::Failure;
use crate::oauth2::models::GoogleAccountInformation;
use crate::oauth2::oauth2_clients::OAUTH2_CLIENTS;
use oauth2::{AuthorizationCode, TokenResponse};
use reqwest::Client;
use uuid::Uuid;

pub struct OAuth2Service;

impl OAuth2Service {
    // Generate state token
    pub fn generate_state() -> String {
        Uuid::now_v7().to_string()
    }

    // Validate state token
    pub fn validate_state(state: &str, expected_state: &str) -> Result<(), Failure> {
        if state != expected_state {
            return Err(Failure::BadRequest("Invalid OAuth2 state".to_string()));
        }
        Ok(())
    }

    pub fn get_google_auth_url() -> (String, String) {
        let state = Self::generate_state();
        let auth_url = OAUTH2_CLIENTS.google_auth_url(&state);
        (auth_url, state)
    }

    pub async fn exchange_google_code(code: &str, state: &str) -> Result<String, Failure> {
        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let pkce_verifier = OAUTH2_CLIENTS
            .get_pkce_verifier(state)
            .ok_or(Failure::BadRequest("PKCE verifier not found for the provided state".to_string()))?;

        let token_result = OAUTH2_CLIENTS
            .google
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&http_client)
            .await
            .map_err(|e| Failure::ExternalServiceError(format!("Failed to exchange Google code: {}", e)))?;

        // Remove the PKCE verifier after use
        OAUTH2_CLIENTS.remove_pkce_verifier(state);
        Ok(token_result.access_token().secret().to_string())
    }

    pub async fn get_google_account_information(access_token: &str) -> Result<GoogleAccountInformation, Failure> {
        let client = Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| Failure::ExternalServiceError(format!("Failed to get Google user info: {}", e)))?;

        if !response.status().is_success() {
            return Err(Failure::ExternalServiceError(format!("Google API returned status: {}", response.status())));
        }

        response
            .json::<GoogleAccountInformation>()
            .await
            .map_err(|e| Failure::ExternalServiceError(format!("Failed to parse Google user info: {}", e)))
    }

    pub fn get_github_auth_url() -> (String, String) {
        let state = Self::generate_state();
        let auth_url = OAUTH2_CLIENTS.github_auth_url(&state);
        (auth_url, state)
    }
}
