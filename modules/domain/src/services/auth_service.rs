use crate::repositories::account_repository::AccountRepository;
use async_trait::async_trait;
use shared::models::failure::Failure;
use std::sync::Arc;

use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordVerifier, password_hash::PasswordHasher};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn hash_password(&self, password: &str) -> Result<String, Failure>;
    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool, Failure>;
}

pub struct AuthServiceImpl {
    repository: Arc<dyn AccountRepository>,
}

impl AuthServiceImpl {
    pub fn new(repository: Arc<dyn AccountRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn hash_password(&self, password: &str) -> Result<String, Failure> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => Ok(hash.to_string()),
            Err(e) => Err(Failure::InternalServerError(format!("Failed to hash password: {}", e))),
        }
    }

    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool, Failure> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| Failure::InternalServerError(format!("Invalid hash format: {}", e)))?;

        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}
