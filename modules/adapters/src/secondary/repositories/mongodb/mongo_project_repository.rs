use async_trait::async_trait;
use mongodb::{Collection, bson::doc};
use std::sync::Arc;

// shared modules
use domain::{entities::project_entity::ProjectEntity, repositories::project_repository::ProjectRepository};
use processors::MongoRepository;
use shared::{models::failure::Failure, types::DomainResponse};

// internal modules
use crate::secondary::repositories::{
    models::project_schema::MongoProjectSchema,
    mongodb::mongo_base_repository::{EntitySchema, MongoBaseRepository},
};

#[derive(MongoRepository)]
pub struct MongoProjectRepository {
    base: MongoBaseRepository<ProjectEntity, MongoProjectSchema>,
}

impl MongoProjectRepository {
    pub fn new(collection: Arc<Collection<MongoProjectSchema>>) -> Self {
        MongoProjectRepository { base: MongoBaseRepository::new(collection) }
    }
}

#[async_trait]
impl ProjectRepository for MongoProjectRepository {
    async fn find_by_name(&self, name: &str) -> DomainResponse<Option<ProjectEntity>> {
        let filter = doc! {
            "name": name,
            "deleted_at": { "$exists": false }
        };

        match self.base.collection.find_one(filter).await {
            Ok(Some(schema)) => Ok(Some(schema.to_entity())),
            Ok(None) => Ok(None),
            Err(e) => Err(Failure::DatabaseError(format!("Failed to find project by name: {}", e))),
        }
    }
}
