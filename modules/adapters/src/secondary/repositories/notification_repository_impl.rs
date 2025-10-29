use async_trait::async_trait;
use domain::repositories::notification_repository::NotificationRepository;

pub struct NotificationRepositoryImpl {}

impl NotificationRepositoryImpl {
    pub fn new() -> Self {
        NotificationRepositoryImpl {}
    }
}

#[async_trait]
impl NotificationRepository for NotificationRepositoryImpl {}
