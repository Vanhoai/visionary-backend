use async_trait::async_trait;

#[async_trait]
pub trait NotificationRepository: Send + Sync {}
