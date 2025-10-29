use async_trait::async_trait;

#[async_trait]
pub trait AuthApi: Send + Sync {}
