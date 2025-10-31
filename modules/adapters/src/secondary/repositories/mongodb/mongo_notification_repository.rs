use async_trait::async_trait;
use domain::repositories::notification_repository::NotificationRepository;

pub struct MongoNotificationRepository {}

impl MongoNotificationRepository {
    pub fn new() -> Self {
        MongoNotificationRepository {}
    }
}

#[async_trait]
impl NotificationRepository for MongoNotificationRepository {}
