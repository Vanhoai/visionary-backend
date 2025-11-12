use config::{Config, Environment};
use dotenv::var;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Arc;

// internal modules
use crate::functions::deserialize_functions::deserialize_comma_separated;
use crate::functions::path_functions::PathFunctions;

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
    pub database_type: String,
    pub mongo_uri: String,
    pub mongo_database: String,
    #[serde(deserialize_with = "deserialize_comma_separated")]
    pub scylla_nodes: Vec<String>,
    pub scylla_keyspace: String,
    pub scylla_replication_factor: u8,
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

impl AppConfig {
    fn path_to_env(mode: &str) -> Result<String, std::env::VarError> {
        let root = PathFunctions::root_path();

        match mode {
            "PRODUCTION" => Ok(format!("{}/environments/.env.production", root)),
            "DEVELOPMENT" => Ok(format!("{}/environments/.env.development", root)),
            "STAGING" => Ok(format!("{}/environments/.env.staging", root)),
            _ => Err(std::env::VarError::NotPresent),
        }
    }

    pub fn from_env() -> Self {
        let mode = var("MODE").unwrap_or_else(|_| "DEVELOPMENT".to_string());
        let env_path = AppConfig::path_to_env(&mode).expect("Can not get env path !!!!");
        dotenv::from_path(env_path).expect("Can not load env file !!!!");

        let config =
            Config::builder().add_source(Environment::default()).build().expect("Can not build env config !!!!");

        let deserialized = config.try_deserialize::<AppConfig>().expect("Can not deserialized production env");

        AppConfig {
            mode: deserialized.mode,
            server: deserialized.server,
            cors: deserialized.cors,
            database: deserialized.database,
            redis: deserialized.redis,
            crypto: deserialized.crypto,
            jwt: deserialized.jwt,
            oauth2: deserialized.oauth2,
        }
    }
}

pub static APP_CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    let config = AppConfig::from_env();
    Arc::new(config)
});
