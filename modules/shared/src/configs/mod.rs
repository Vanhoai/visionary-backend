use crate::configs::configs::AppConfig;
use crate::functions::path_functions::PathFunctions;
use config::{Config, Environment};
use dotenv::var;
use once_cell::sync::Lazy;
use std::sync::Arc;

pub mod configs;

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
        }
    }
}

pub static APP_CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    let config = AppConfig::from_env();
    Arc::new(config)
});
