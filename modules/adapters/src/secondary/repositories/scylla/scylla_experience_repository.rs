use async_trait::async_trait;
use scylla::client::session::Session;
use std::sync::Arc;

// shared modules
use domain::entities::experience_entity::ExperienceEntity;
use domain::repositories::experience_repository::ExperienceRepository;
use shared::types::DomainResponse;

// internal modules
use crate::impl_scylla_base_repository;
use crate::secondary::repositories::models::experience_schema::ScyllaExperienceSchema;
use crate::secondary::repositories::scylla::scylla_base_repository::ScyllaBaseRepository;

#[allow(dead_code)]
pub struct ScyllaExperienceRepository {
    base: ScyllaBaseRepository<ExperienceEntity, ScyllaExperienceSchema>,
}

impl ScyllaExperienceRepository {
    pub fn new(session: Arc<Session>, keyspace: &str) -> Self {
        ScyllaExperienceRepository { base: ScyllaBaseRepository::new(session, keyspace) }
    }
}

impl_scylla_base_repository!(ScyllaExperienceRepository, ExperienceEntity, ScyllaExperienceSchema);

#[async_trait]
impl ExperienceRepository for ScyllaExperienceRepository {
    async fn find_by_account_id(&self, _account_id: &str) -> DomainResponse<Vec<ExperienceEntity>> {
        todo!()
    }

    async fn find_by_company(&self, _company: &str) -> DomainResponse<Option<ExperienceEntity>> {
        todo!()
    }
}
