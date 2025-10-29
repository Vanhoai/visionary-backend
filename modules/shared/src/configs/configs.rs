use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: u8,
    pub password: String,
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
    pub access_duration: String,
    pub refresh_duration: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub mode: String,
    pub server: ServerConfig,
    pub redis: RedisConfig,
    pub crypto: CryptoConfig,
    pub jwt: JwtConfig,
}

#[derive(Deserialize, Debug)]
pub struct DevelopmentConfig {
    pub mode: String,
    pub dev_server: ServerConfig,
    pub dev_redis: RedisConfig,
    pub dev_crypto: CryptoConfig,
    pub dev_jwt: JwtConfig,
}

#[derive(Deserialize, Debug)]
pub struct ProductionConfig {
    pub mode: String,
    pub server: ServerConfig,
    pub redis: RedisConfig,
    pub crypto: CryptoConfig,
    pub jwt: JwtConfig,
}
