use async_trait::async_trait;
use shared::types::DomainResponse;
use crate::usecases::notification_usecases::PushNotification;

#[async_trait]
pub trait MessagingApi: Send + Sync {
    async fn send_push_notification(&self, token: &str, notification: &PushNotification) -> DomainResponse<String>;
    async fn send_push_to_topic(&self, topic: &str, notification: &PushNotification) -> DomainResponse<String>;
}
