use crate::entities::work_entity::WorkEntity;
use crate::repositories::work_repository::WorkRepository;
use async_trait::async_trait;
use shared::models::filters::MongoFilter;
use shared::types::DomainResponse;
use std::sync::Arc;

#[async_trait]
pub trait WorkService: Send + Sync {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<WorkEntity>>;
}

pub struct WorkServiceImpl {
    repository: Arc<dyn WorkRepository<Filter = MongoFilter>>,
}

impl WorkServiceImpl {
    pub fn new(repository: Arc<dyn WorkRepository<Filter = MongoFilter>>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl WorkService for WorkServiceImpl {
    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<WorkEntity>> {
        self.repository.find_by_account_id(account_id).await
    }
}
