use async_trait::async_trait;
use mongodb::Collection;
use std::sync::Arc;

// shared modules
use domain::entities::blog_entity::BlogEntity;
use domain::repositories::blog_repository::BlogRepository;
use processors::MongoRepository;

// internal modules
use crate::secondary::repositories::mongodb::mongo_base_repository::MongoBaseRepository;
use crate::secondary::repositories::mongodb::schemas::blog_schema::BlogSchema;

#[derive(MongoRepository)]
pub struct MongoBlogRepository {
    base: MongoBaseRepository<BlogEntity, BlogSchema>,
}

impl MongoBlogRepository {
    pub fn new(collection: Arc<Collection<BlogSchema>>) -> Self {
        MongoBlogRepository { base: MongoBaseRepository::new(collection) }
    }
}

#[async_trait]
impl BlogRepository for MongoBlogRepository {}
