use crate::impl_mongo_base_repository;
use crate::secondary::repositories::mongodb::mongo_base_repository::MongoBaseRepository;
use crate::secondary::repositories::mongodb::schemas::provider_schema::ProviderSchema;
use async_trait::async_trait;
use domain::entities::provider_entity::ProviderEntity;
use domain::repositories::provider_repository::ProviderRepository;
use mongodb::Collection;
use std::sync::Arc;

pub struct MongoProviderRepository {
    base: MongoBaseRepository<ProviderEntity, ProviderSchema>,
}

impl MongoProviderRepository {
    pub fn new(collection: Arc<Collection<ProviderSchema>>) -> Self {
        MongoProviderRepository { base: MongoBaseRepository::new(collection) }
    }
}

impl_mongo_base_repository!(MongoProviderRepository, ProviderEntity, ProviderSchema);

#[async_trait]
impl ProviderRepository for MongoProviderRepository {}
