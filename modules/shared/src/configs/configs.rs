use crate::functions::deserialize_functions::deserialize_comma_separated;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct CorsConfig {
    pub enabled: bool,
    #[serde(deserialize_with = "deserialize_comma_separated")]
    pub allow_origins: Vec<String>,
    #[serde(deserialize_with = "deserialize_comma_separated")]
    pub allow_headers: Vec<String>,
    #[serde(deserialize_with = "deserialize_comma_separated")]
    pub allow_methods: Vec<String>,
    pub allow_credentials: bool,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: u8,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub mongo_uri: String,
    pub mongo_database: String,
}

#[derive(Debug, Deserialize)]
pub struct CryptoConfig {
    pub algorithm_type: String,
    pub algorithm: String,
    pub key_size: Option<u32>,
    pub curve: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JwtConfig {
    pub access_token_expiry: i64,
    pub refresh_token_expiry: i64,
}

#[derive(Debug, Deserialize)]
pub struct OAuth2Config {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_redirect_url: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub mode: String,
    pub server: ServerConfig,
    pub cors: CorsConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub crypto: CryptoConfig,
    pub jwt: JwtConfig,
    pub oauth2: OAuth2Config,
}
