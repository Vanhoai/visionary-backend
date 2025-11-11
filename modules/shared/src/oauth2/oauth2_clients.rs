use config::Map;
use oauth2::basic::{BasicClient, BasicErrorResponseType, BasicTokenType};
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenUrl,
};
use oauth2::{
    Client, EmptyExtraTokenFields, EndpointNotSet, EndpointSet, RevocationErrorResponseType, StandardErrorResponse,
    StandardRevocableToken, StandardTokenIntrospectionResponse, StandardTokenResponse,
};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

// internal modules
use crate::configs::APP_CONFIG;

type OAuth2Client = Client<
    StandardErrorResponse<BasicErrorResponseType>,
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
    StandardRevocableToken,
    StandardErrorResponse<RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

pub struct OAuth2Clients {
    pub google: OAuth2Client,
    pub github: OAuth2Client,
}

impl OAuth2Clients {
    pub fn new() -> Self {
        println!("Google Redirect URL: {}", APP_CONFIG.oauth2.google_redirect_url);

        let google = BasicClient::new(ClientId::new(APP_CONFIG.oauth2.google_client_id.clone()))
            .set_client_secret(ClientSecret::new(APP_CONFIG.oauth2.google_client_secret.clone()))
            .set_auth_uri(AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap())
            .set_token_uri(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap())
            .set_redirect_uri(RedirectUrl::new(APP_CONFIG.oauth2.google_redirect_url.clone()).unwrap());

        let github = BasicClient::new(ClientId::new(APP_CONFIG.oauth2.github_client_id.clone()))
            .set_client_secret(ClientSecret::new(APP_CONFIG.oauth2.github_client_secret.clone()))
            .set_auth_uri(AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap())
            .set_token_uri(TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap())
            .set_redirect_uri(RedirectUrl::new(APP_CONFIG.oauth2.github_redirect_url.clone()).unwrap());

        Self { google, github }
    }

    fn store_pkce_verifier(&self, state: &str, pkce_verifier: PkceCodeVerifier) {
        let mut verifiers = PKCE_VERIFIERS.lock().unwrap();
        verifiers.insert(state.to_string(), pkce_verifier.secret().to_string());
    }

    pub fn get_pkce_verifier(&self, state: &str) -> Option<PkceCodeVerifier> {
        let verifiers = PKCE_VERIFIERS.lock().unwrap();
        verifiers.get(state).map(|v| PkceCodeVerifier::new(v.to_string()))
    }

    pub fn remove_pkce_verifier(&self, state: &str) {
        let mut verifiers = PKCE_VERIFIERS.lock().unwrap();
        verifiers.remove(state);
    }

    // Generate authorization URL for Google
    pub fn google_auth_url(&self, state: &str) -> String {
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        self.store_pkce_verifier(state, pkce_verifier);

        let (auth_url, _csrf_token) = self
            .google
            .authorize_url(|| CsrfToken::new(state.to_string()))
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        auth_url.to_string()
    }

    // Generate authorization URL cho GitHub
    pub fn github_auth_url(&self, state: &str) -> String {
        let (auth_url, _csrf_token) = self
            .github
            .authorize_url(|| CsrfToken::new(state.to_string()))
            .add_scope(Scope::new("user:email".to_string()))
            .add_scope(Scope::new("read:user".to_string()))
            .url();

        auth_url.to_string()
    }
}

// Global singleton
pub static PKCE_VERIFIERS: Lazy<Arc<Mutex<Map<String, String>>>> = Lazy::new(|| Arc::new(Mutex::new(Map::new())));
pub static OAUTH2_CLIENTS: Lazy<Arc<OAuth2Clients>> = Lazy::new(|| Arc::new(OAuth2Clients::new()));
