use async_trait::async_trait;
use crate::entities::blog_entity::BlogEntity;
use crate::repositories::base_repository::BaseRepository;

#[async_trait]
pub trait BlogRepository: BaseRepository<BlogEntity> {}