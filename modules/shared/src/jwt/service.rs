use crate::configs::APP_CONFIG;
use crate::jwt::claims::Claims;
use crate::jwt::keys::KEY_MANAGER;
use chrono::Duration;
use jsonwebtoken::{Header, TokenData, Validation, decode, encode};

pub struct JwtService;

impl JwtService {
    pub fn generate_access_token(account_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = chrono::Utc::now();
        let expiry = now + Duration::seconds(APP_CONFIG.jwt.access_token_expiry);
        let claims = Claims { sub: account_id.to_string(), exp: expiry.timestamp(), iat: now.timestamp() };
        
        let header = Header::new(KEY_MANAGER.access_keys.algorithm);
        encode(&header, &claims, &KEY_MANAGER.access_keys.encoding_key)
    }

    pub fn generate_refresh_token(account_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = chrono::Utc::now();
        let expiry = now + Duration::seconds(APP_CONFIG.jwt.refresh_token_expiry);
        let claims = Claims { sub: account_id.to_string(), exp: expiry.timestamp(), iat: now.timestamp() };

        let header = Header::new(KEY_MANAGER.refresh_keys.algorithm);
        encode(&header, &claims, &KEY_MANAGER.refresh_keys.encoding_key)
    }

    pub fn verify_access_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(KEY_MANAGER.access_keys.algorithm);
        validation.validate_exp = true;

        decode::<Claims>(token, &KEY_MANAGER.access_keys.decoding_key, &validation)
    }

    pub fn verify_refresh_token(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(KEY_MANAGER.refresh_keys.algorithm);
        validation.validate_exp = true;

        decode::<Claims>(token, &KEY_MANAGER.refresh_keys.decoding_key, &validation)
    }
}
