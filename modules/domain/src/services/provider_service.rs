use crate::repositories::provider_repository::ProviderRepository;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait ProviderService: Send + Sync {}

pub struct ProviderServiceImpl {
    repository: Arc<dyn ProviderRepository>,
}

impl ProviderServiceImpl {
    pub fn new(repository: Arc<dyn ProviderRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ProviderService for ProviderServiceImpl {}
