use crate::services::notification_service::NotificationService;
use std::sync::Arc;

pub struct NotificationAppService {
    notification_service: Arc<dyn NotificationService>,
}

impl NotificationAppService {
    pub fn new(notification_service: Arc<dyn NotificationService>) -> Self {
        Self { notification_service }
    }
}
