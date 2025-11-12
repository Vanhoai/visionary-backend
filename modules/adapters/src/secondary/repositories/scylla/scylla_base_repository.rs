use async_trait::async_trait;
use scylla::client::session::Session;
use std::marker::PhantomData;
use std::sync::Arc;

// shared modules
use domain::repositories::base_repository::BaseRepository;
use shared::models::paginate::Paginate;
use shared::types::DomainResponse;

pub trait EntitySchema<E>: Send + Sync + Clone {
    /// Convert entity to schema for database operations
    fn from_entity(entity: &E) -> Self;

    /// Convert schema back to entity after database operations
    fn to_entity(&self) -> E;
}

pub struct ScyllaBaseRepository<E, S>
where
    E: Send + Sync,
    S: EntitySchema<E>,
{
    pub session: Arc<Session>,
    pub keyspace: String,
    _phantom: PhantomData<(E, S)>,
}

impl<E, S> ScyllaBaseRepository<E, S>
where
    E: Send + Sync,
    S: EntitySchema<E>,
{
    pub fn new(session: Arc<Session>, keyspace: &str) -> Self {
        ScyllaBaseRepository { session, keyspace: keyspace.to_string(), _phantom: PhantomData }
    }
}
#[macro_export]
macro_rules! impl_scylla_base_repository {
    ($repository:ty, $entity:ty, $schema:ty) => {
        #[async_trait::async_trait]
        impl domain::repositories::base_repository::BaseRepository<$entity> for $repository {
            async fn create(&self, entity: &$entity) -> shared::types::DomainResponse<$entity> {
                self.base.create(entity).await
            }

            async fn update(&self, id: &str, entity: &$entity) -> shared::types::DomainResponse<$entity> {
                self.base.update(id, entity).await
            }

            async fn delete(&self, id: &str) -> shared::types::DomainResponse<$entity> {
                self.base.delete(id).await
            }

            async fn remove(&self, id: &str) -> shared::types::DomainResponse<$entity> {
                self.base.remove(id).await
            }

            async fn find(&self, id: &str) -> shared::types::DomainResponse<Option<$entity>> {
                self.base.find(id).await
            }

            async fn find_and_delete(&self, id: &str) -> shared::types::DomainResponse<$entity> {
                self.base.find_and_delete(id).await
            }

            async fn find_and_remove(&self, id: &str) -> shared::types::DomainResponse<$entity> {
                self.base.find_and_remove(id).await
            }

            async fn finds(&self) -> shared::types::DomainResponse<Vec<$entity>> {
                self.base.finds().await
            }

            async fn finds_paginated(
                &self,
                page: u32,
                page_size: u32,
            ) -> shared::types::DomainResponse<(shared::models::paginate::Paginate, Vec<$entity>)> {
                self.base.finds_paginated(page, page_size).await
            }
        }
    };
}

#[async_trait]
impl<E, S> BaseRepository<E> for ScyllaBaseRepository<E, S>
where
    E: Send + Sync,
    S: EntitySchema<E>,
{
    async fn create(&self, _entity: &E) -> DomainResponse<E> {
        todo!()
    }

    async fn update(&self, _id: &str, _entity: &E) -> DomainResponse<E> {
        todo!()
    }

    async fn delete(&self, _id: &str) -> DomainResponse<E> {
        todo!()
    }

    async fn remove(&self, _id: &str) -> DomainResponse<E> {
        todo!()
    }

    async fn find(&self, _id: &str) -> DomainResponse<Option<E>> {
        todo!()
    }

    async fn find_and_delete(&self, _id: &str) -> DomainResponse<E> {
        todo!()
    }

    async fn find_and_remove(&self, _id: &str) -> DomainResponse<E> {
        todo!()
    }

    async fn finds(&self) -> DomainResponse<Vec<E>> {
        todo!()
    }

    async fn finds_paginated(&self, _page: u32, _page_size: u32) -> DomainResponse<(Paginate, Vec<E>)> {
        todo!()
    }
}
