use crate::configs::configs::{AppConfig, DevelopmentConfig, ProductionConfig};
use config::{Config, Environment};
use dotenv::var;
use once_cell::sync::Lazy;
use std::sync::Arc;

pub mod configs;

impl AppConfig {
    pub fn from_env() -> Self {
        let mode = var("MODE").unwrap_or_else(|_| "DEVELOPMENT".to_string());
        let config: Config = Config::builder()
            .add_source(Environment::default())
            .build()
            .expect("Can not build env config !!!!");

        match mode.as_str() {
            "PRODUCTION" => {
                let deserialized = config
                    .try_deserialize::<ProductionConfig>()
                    .expect("Can not deserialized production env");

                AppConfig {
                    mode: deserialized.mode,
                    server: deserialized.server,
                    redis: deserialized.redis,
                    crypto: deserialized.crypto,
                    jwt: deserialized.jwt,
                }
            },
            _ => {
                let deserialized = config
                    .try_deserialize::<DevelopmentConfig>()
                    .expect("Can not deserialized production env");

                AppConfig {
                    mode: deserialized.mode,
                    server: deserialized.dev_server,
                    redis: deserialized.dev_redis,
                    crypto: deserialized.dev_crypto,
                    jwt: deserialized.dev_jwt,
                }
            },
        }
    }
}

pub static APP_CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    let config = AppConfig::from_env();
    Arc::new(config)
});
