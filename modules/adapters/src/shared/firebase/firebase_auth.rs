use base64::{Engine as _, engine::general_purpose};
use reqwest::Client;
use serde::Deserialize;
use shared::models::failure::Failure;

#[derive(Deserialize)]
struct GooglePublicKey {
    keys: Vec<JWK>,
}

#[derive(Deserialize)]
struct JWK {
    kid: String,
    n: String,
    e: String,
}

#[derive(Deserialize)]
struct TokenHeader {
    kid: String,
}

#[derive(Deserialize)]
struct TokenPayload {
    sub: String,
    email: String,
    name: Option<String>,
    picture: Option<String>,
    email_verified: bool,
    aud: String,
    exp: i64,
}

pub struct FirebaseAuth {
    client: Client,
}

impl FirebaseAuth {
    async fn get_google_public_keys(&self) -> Result<GooglePublicKey, Failure> {
        let url = "https://www.googleapis.com/oauth2/v3/certs";

        self.client
            .get(url)
            .send()
            .await
            .map_err(|e| Failure::ExternalServiceError(format!("Failed to fetch Google public keys: {}", e)))?
            .json::<GooglePublicKey>()
            .await
            .map_err(|e| Failure::ExternalServiceError(format!("Failed to parse public keys: {}", e)))
    }

    fn decode_token_parts(&self, token: &str) -> Result<(TokenHeader, TokenPayload), Failure> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(Failure::BadRequest("Invalid token format".to_string()));
        }

        // Decode header
        let header_json = general_purpose::URL_SAFE_NO_PAD
            .decode(parts[0])
            .map_err(|e| Failure::BadRequest(format!("Failed to decode header: {}", e)))?;
        let header: TokenHeader = serde_json::from_slice(&header_json)
            .map_err(|e| Failure::BadRequest(format!("Failed to parse header: {}", e)))?;

        // Decode payload
        let payload_json = general_purpose::URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|e| Failure::BadRequest(format!("Failed to decode payload: {}", e)))?;
        let payload: TokenPayload = serde_json::from_slice(&payload_json)
            .map_err(|e| Failure::BadRequest(format!("Failed to parse payload: {}", e)))?;

        Ok((header, payload))
    }
}
