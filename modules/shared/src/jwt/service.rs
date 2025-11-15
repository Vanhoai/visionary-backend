use chrono::Duration;
use jsonwebtoken::{Header, TokenData, Validation, decode, encode};

// internal modules
use crate::configs::APP_CONFIG;
use crate::jwt::claims::Claims;
use crate::jwt::keys::KEY_MANAGER;
use crate::models::failure::Failure;

pub struct JwtService;

impl JwtService {
    pub fn generate_access_token(account_id: &str, jti: &str, role: Option<String>) -> Result<String, Failure> {
        let now = chrono::Utc::now();
        let expiry = now + Duration::seconds(APP_CONFIG.jwt.access_token_expiry);
        let claims = Claims {
            sub: account_id.to_string(),
            jti: jti.to_string(),
            exp: expiry.timestamp(),
            iat: now.timestamp(),
            role,
        };

        let header = Header::new(KEY_MANAGER.access_keys.algorithm);

        encode(&header, &claims, &KEY_MANAGER.access_keys.encoding_key)
            .map_err(|e| Failure::InternalServerError(format!("Failed to generate access token: {}", e)))
    }

    pub fn generate_refresh_token(account_id: &str, jti: &str, role: Option<String>) -> Result<String, Failure> {
        let now = chrono::Utc::now();
        let expiry = now + Duration::seconds(APP_CONFIG.jwt.refresh_token_expiry);
        let claims = Claims {
            sub: account_id.to_string(),
            jti: jti.to_string(),
            exp: expiry.timestamp(),
            iat: now.timestamp(),
            role,
        };

        let header = Header::new(KEY_MANAGER.refresh_keys.algorithm);
        encode(&header, &claims, &KEY_MANAGER.refresh_keys.encoding_key)
            .map_err(|e| Failure::InternalServerError(format!("Failed to generate refresh token: {}", e)))
    }

    pub fn verify_access_token(token: &str) -> Result<TokenData<Claims>, Failure> {
        let mut validation = Validation::new(KEY_MANAGER.access_keys.algorithm);
        validation.validate_exp = true;

        decode::<Claims>(token, &KEY_MANAGER.access_keys.decoding_key, &validation)
            .map_err(|e| Failure::Unauthorized(format!("Invalid access token: {}", e)))
    }

    pub fn verify_refresh_token(token: &str) -> Result<TokenData<Claims>, Failure> {
        let mut validation = Validation::new(KEY_MANAGER.refresh_keys.algorithm);
        validation.validate_exp = true;

        decode::<Claims>(token, &KEY_MANAGER.refresh_keys.decoding_key, &validation)
            .map_err(|e| Failure::Unauthorized(format!("Invalid refresh token: {}", e)))
    }
}
