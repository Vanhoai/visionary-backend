use crate::secondary::repositories::mongodb::mongo_base_repository::MongoBaseRepository;
use crate::secondary::repositories::mongodb::schemas::blog_schema::BlogSchema;
use async_trait::async_trait;
use domain::entities::blog_entity::BlogEntity;
use domain::repositories::blog_repository::BlogRepository;
use mongodb::Collection;
use processors::MongoRepository;
use std::sync::Arc;

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
