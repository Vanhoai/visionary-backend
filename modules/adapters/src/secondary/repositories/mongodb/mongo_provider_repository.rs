use async_trait::async_trait;
use domain::repositories::provider_repository::ProviderRepository;

pub struct MongoProviderRepository {}

impl MongoProviderRepository {
    pub fn new() -> Self {
        MongoProviderRepository {}
    }
}

#[async_trait]
impl ProviderRepository for MongoProviderRepository {}
