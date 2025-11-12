use async_trait::async_trait;
use mongodb::Collection;
use std::sync::Arc;

// shared modules
use domain::entities::notification_entity::NotificationEntity;
use domain::repositories::notification_repository::NotificationRepository;
use processors::MongoRepository;

use crate::secondary::repositories::models::notification_schema::MongoNotificationSchema;
// shared modules
use crate::secondary::repositories::mongodb::mongo_base_repository::MongoBaseRepository;

#[derive(MongoRepository)]
pub struct MongoNotificationRepository {
    base: MongoBaseRepository<NotificationEntity, MongoNotificationSchema>,
}

impl MongoNotificationRepository {
    pub fn new(collection: Arc<Collection<MongoNotificationSchema>>) -> Self {
        MongoNotificationRepository { base: MongoBaseRepository::new(collection) }
    }
}

#[async_trait]
impl NotificationRepository for MongoNotificationRepository {}
