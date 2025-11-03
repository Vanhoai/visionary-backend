use async_trait::async_trait;
use domain::repositories::base_repository::BaseRepository;
use mongodb::Collection;
use serde::Serialize;
use serde::de::DeserializeOwned;
use shared::models::failure::Failure;
use shared::types::DomainResponse;
use std::marker::PhantomData;
use std::sync::Arc;

pub trait EntitySchema<S>: Serialize + DeserializeOwned + Unpin + Send + Sync {
    fn from_entity(entity: S) -> Self;
    fn to_entity(&self) -> S;
}

pub struct MongoBaseRepository<E, S>
where
    E: Send + Sync + Clone,
    S: EntitySchema<E>,
{
    pub collection: Arc<Collection<S>>,
    _phantom: PhantomData<E>,
}

impl<E, S> MongoBaseRepository<E, S>
where
    E: Send + Sync + Clone,
    S: EntitySchema<E>,
{
    pub fn new(collection: Arc<Collection<S>>) -> Self {
        MongoBaseRepository { collection, _phantom: PhantomData }
    }
}

#[macro_export]
macro_rules! impl_mongo_base_repository {
    ($repository:ty, $entity:ty, $schema:ty) => {
        #[async_trait::async_trait]
        impl domain::repositories::base_repository::BaseRepository<$entity> for $repository {
            async fn create(&self, entity: $entity) -> shared::types::DomainResponse<$entity> {
                self.base.create(entity).await
            }

            async fn update(&self, id: &str, entity: $entity) -> shared::types::DomainResponse<$entity> {
                self.base.update(id, entity).await
            }

            async fn delete(&self, id: &str) -> shared::types::DomainResponse<$entity> {
                self.base.delete(id).await
            }

            async fn find_by_id(&self, id: &str) -> shared::types::DomainResponse<Option<$entity>> {
                self.base.find_by_id(id).await
            }
        }
    };
}

#[async_trait]
impl<E, S> BaseRepository<E> for MongoBaseRepository<E, S>
where
    E: Send + Sync + Clone,
    S: EntitySchema<E>,
{
    async fn create(&self, entity: E) -> DomainResponse<E> {
        let schema = S::from_entity(entity.clone());
        self.collection
            .insert_one(&schema)
            .await
            .map_err(|e| Failure::DatabaseError(format!("Failed to create entity: {}", e)))?;

        Ok(schema.to_entity())
    }

    async fn update(&self, id: &str, entity: E) -> DomainResponse<E> {
        todo!()
    }

    async fn delete(&self, id: &str) -> DomainResponse<E> {
        todo!()
    }

    async fn find_by_id(&self, id: &str) -> DomainResponse<Option<E>> {
        todo!()
    }
}
