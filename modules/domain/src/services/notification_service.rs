use async_trait::async_trait;
use std::sync::Arc;

// internal modules
use crate::repositories::notification_repository::NotificationRepository;

#[async_trait]
pub trait NotificationService: Send + Sync {}

#[allow(dead_code)]
pub struct NotificationServiceImpl {
    repository: Arc<dyn NotificationRepository>,
}

impl NotificationServiceImpl {
    pub fn new(repository: Arc<dyn NotificationRepository>) -> Self {
        Self { repository: repository }
    }
}

#[async_trait]
impl NotificationService for NotificationServiceImpl {}
