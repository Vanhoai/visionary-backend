use async_trait::async_trait;

// internal modules
use crate::entities::notification_entity::NotificationEntity;
use crate::repositories::base_repository::BaseRepository;

#[async_trait]
pub trait NotificationRepository: BaseRepository<NotificationEntity> {}
