use async_trait::async_trait;
use mongodb::Collection;
use std::sync::Arc;

// shared modules
use domain::entities::category_entity::CategoryEntity;
use domain::repositories::category_repository::CategoryRepository;
use processors::MongoRepository;
use shared::types::DomainResponse;

// internal modules
use crate::secondary::repositories::{
    models::category_schema::MongoCategorySchema,
    mongodb::mongo_base_repository::{EntitySchema, MongoBaseRepository},
};

#[derive(MongoRepository)]
pub struct MongoCategoryRepository {
    base: MongoBaseRepository<CategoryEntity, MongoCategorySchema>,
}

impl MongoCategoryRepository {
    pub fn new(collection: Arc<Collection<MongoCategorySchema>>) -> Self {
        MongoCategoryRepository { base: MongoBaseRepository::new(collection) }
    }
}

#[async_trait]
impl CategoryRepository for MongoCategoryRepository {
    async fn find_by_name(&self, name: &str) -> DomainResponse<Option<CategoryEntity>> {
        let filter = mongodb::bson::doc! {
            "name": name,
            "deleted_at": { "$exists": false }
        };

        match self.base.collection.find_one(filter).await {
            Ok(Some(schema)) => Ok(Some(schema.to_entity())),
            Ok(None) => Ok(None),
            Err(e) => {
                Err(shared::models::failure::Failure::DatabaseError(format!("Failed to find category by name: {}", e)))
            },
        }
    }
}
