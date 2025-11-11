use async_trait::async_trait;

// shared modules
use shared::types::DomainResponse;

#[async_trait]
pub trait AuthApi: Send + Sync {
    async fn verify_google_token(&self, id_token: &str) -> DomainResponse<()>;
}
