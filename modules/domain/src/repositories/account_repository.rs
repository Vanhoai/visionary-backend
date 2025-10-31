use async_trait::async_trait;

#[async_trait]
pub trait AccountRepository: Send + Sync {}
