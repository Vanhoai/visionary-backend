use crate::{
    entities::provider_entity::ProviderEntity, repositories::provider_repository::ProviderRepository,
    values::auth_provider::AuthProvider,
};
use async_trait::async_trait;
use shared::types::DomainResponse;
use std::sync::Arc;

#[async_trait]
pub trait ProviderService: Send + Sync {
    async fn create_provider(
        &self,
        account_id: String,
        provider: String,
        identify: String,
    ) -> DomainResponse<ProviderEntity>;

    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<ProviderEntity>>;
}

pub struct ProviderServiceImpl {
    repository: Arc<dyn ProviderRepository>,
}

impl ProviderServiceImpl {
    pub fn new(repository: Arc<dyn ProviderRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ProviderService for ProviderServiceImpl {
    async fn create_provider(
        &self,
        account_id: String,
        provider: String,
        identify: String,
    ) -> DomainResponse<ProviderEntity> {
        let auth_provider = AuthProvider::from_string(&provider)?;
        let provider = ProviderEntity::new(false, account_id, auth_provider, identify);

        self.repository.create(provider).await
    }

    async fn find_by_account_id(&self, account_id: &str) -> DomainResponse<Vec<ProviderEntity>> {
        self.repository.find_by_account_id(&account_id).await
    }
}
