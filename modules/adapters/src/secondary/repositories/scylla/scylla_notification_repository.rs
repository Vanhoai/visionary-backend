use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;

// shared modules
use domain::entities::notification_entity::NotificationEntity;
use domain::repositories::notification_repository::NotificationRepository;

// internal modules
use crate::impl_scylla_base_repository;
use crate::secondary::repositories::models::notification_schema::ScyllaNotificationSchema;
use crate::secondary::repositories::scylla::scylla_base_repository::ScyllaBaseRepository;

#[allow(dead_code)]
pub struct ScyllaNotificationRepository {
    base: ScyllaBaseRepository<NotificationEntity, ScyllaNotificationSchema>,
}

impl ScyllaNotificationRepository {
    pub fn new(session: Arc<Session>, keyspace: &str, table_name: &str) -> Self {
        ScyllaNotificationRepository { base: ScyllaBaseRepository::new(session, keyspace, table_name) }
    }
}

impl_scylla_base_repository!(ScyllaNotificationRepository, NotificationEntity, ScyllaNotificationSchema);

#[async_trait]
impl NotificationRepository for ScyllaNotificationRepository {}
