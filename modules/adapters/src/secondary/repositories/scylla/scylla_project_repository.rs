use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;

// shared modules
use domain::{entities::project_entity::ProjectEntity, repositories::project_repository::ProjectRepository};
use shared::types::DomainResponse;

// internal modules
use crate::{
    impl_scylla_base_repository,
    secondary::repositories::{
        models::project_schema::ScyllaProjectSchema, scylla::scylla_base_repository::ScyllaBaseRepository,
    },
};

#[allow(dead_code)]
pub struct ScyllaProjectRepository {
    base: ScyllaBaseRepository<ProjectEntity, ScyllaProjectSchema>,
}

impl ScyllaProjectRepository {
    pub fn new(session: Arc<Session>, keyspace: &str, table_name: &str) -> Self {
        ScyllaProjectRepository { base: ScyllaBaseRepository::new(session, keyspace, table_name) }
    }
}

impl_scylla_base_repository!(ScyllaProjectRepository, ProjectEntity, ScyllaProjectSchema);

#[async_trait]
impl ProjectRepository for ScyllaProjectRepository {
    async fn find_by_name(&self, _name: &str) -> DomainResponse<Option<ProjectEntity>> {
        todo!()
    }

    async fn update_project_partial(
        &self,
        _id: &str,
        _cover: Option<String>,
        _name: Option<String>,
        _description: Option<String>,
        _link: Option<String>,
        _github: Option<String>,
        _tags: Option<Vec<String>>,
        _markdown: Option<String>,
    ) -> DomainResponse<ProjectEntity> {
        unimplemented!()
    }
}
