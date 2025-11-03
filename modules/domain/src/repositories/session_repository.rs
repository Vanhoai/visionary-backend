use async_trait::async_trait;

#[async_trait]
pub trait SessionRepository: Send + Sync {}
