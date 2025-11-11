use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// shared modules
use shared::types::DomainResponse;

// ================================ PUSH NOTIFICATION ================================
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushNotification {
    pub title: String,
    pub body: String,
    pub data: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendNotificationResponse {
    pub message_id: String,
}

#[async_trait]
pub trait PushNotificationUseCase: Send + Sync {
    async fn send_to_account(
        &self,
        account_id: &str,
        params: &PushNotification,
    ) -> DomainResponse<SendNotificationResponse>;

    async fn send_to_topic(&self, topic: &str, params: &PushNotification) -> DomainResponse<SendNotificationResponse>;
}
// ================================ PUSH NOTIFICATION ================================

// ================================ MANAGE NOTIFICATION ================================
#[async_trait]
pub trait MangeNotificationUseCase: Send + Sync {}
// ================================ MANAGE NOTIFICATION ================================
