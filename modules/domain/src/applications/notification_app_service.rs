use std::sync::Arc;

// internal modules
use crate::services::notification_service::NotificationService;

#[allow(dead_code)]
pub struct NotificationAppService {
    notification_service: Arc<dyn NotificationService>,
}

impl NotificationAppService {
    pub fn new(notification_service: Arc<dyn NotificationService>) -> Self {
        Self { notification_service }
    }
}
