use async_trait::async_trait;

// ================================ MANAGE NOTIFICATION ================================

#[async_trait]
pub trait MangeNotificationUseCases: Send + Sync {}

// ================================ MANAGE NOTIFICATION ================================

// ================================ PUSH NOTIFICATION ================================

#[async_trait]
pub trait PushNotificationUseCases: Send + Sync {}

// ================================ PUSH NOTIFICATION ================================