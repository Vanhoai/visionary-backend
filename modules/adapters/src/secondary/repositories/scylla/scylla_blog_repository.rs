use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;

// shared modules
use domain::entities::blog_entity::BlogEntity;
use domain::repositories::blog_repository::BlogRepository;

// internal modules
use crate::impl_scylla_base_repository;
use crate::secondary::repositories::models::blog_schema::ScyllaBlogSchema;
use crate::secondary::repositories::scylla::scylla_base_repository::ScyllaBaseRepository;

#[allow(dead_code)]
pub struct ScyllaBlogRepository {
    base: ScyllaBaseRepository<BlogEntity, ScyllaBlogSchema>,
}

impl ScyllaBlogRepository {
    pub fn new(session: Arc<Session>, keyspace: &str) -> Self {
        ScyllaBlogRepository { base: ScyllaBaseRepository::new(session, keyspace) }
    }
}

impl_scylla_base_repository!(ScyllaBlogRepository, BlogEntity, ScyllaBlogSchema);

#[async_trait]
impl BlogRepository for ScyllaBlogRepository {}
