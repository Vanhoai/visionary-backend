use async_trait::async_trait;
use domain::repositories::provider_repository::ProviderRepository;

pub struct ProviderRepositoryImpl {}

impl ProviderRepositoryImpl {
    pub fn new() -> Self {
        ProviderRepositoryImpl {}
    }
}

#[async_trait]
impl ProviderRepository for ProviderRepositoryImpl {}