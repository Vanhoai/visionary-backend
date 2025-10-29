use async_trait::async_trait;

#[async_trait]
pub trait ProviderRepository: Send + Sync {}
