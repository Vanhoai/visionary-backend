use async_trait::async_trait;
use scylla::_macro_internal::SerializeRow;
use scylla::client::session::Session;
use scylla::statement::prepared::PreparedStatement;
use std::marker::PhantomData;
use std::sync::Arc;
use std::vec;

// shared modules
use domain::repositories::base_repository::BaseRepository;
use shared::models::failure::Failure;
use shared::models::paginate::Paginate;
use shared::types::DomainResponse;

pub trait EntitySchema<E>: Send + Sync + Clone + SerializeRow {
    fn from_entity(entity: &E) -> Self;
    fn to_entity(&self) -> E;
    fn columns() -> &'static str;
    fn insert_placeholders() -> &'static str;
}

pub struct ScyllaBaseRepository<E, S>
where
    E: Send + Sync,
    S: EntitySchema<E>,
{
    pub session: Arc<Session>,
    pub keyspace: String,
    pub table_name: String,
    _phantom: PhantomData<(E, S)>,
}

impl<E, S> ScyllaBaseRepository<E, S>
where
    E: Send + Sync,
    S: EntitySchema<E>,
{
    pub fn new(session: Arc<Session>, keyspace: &str, table_name: &str) -> Self {
        ScyllaBaseRepository {
            session,
            keyspace: keyspace.to_string(),
            table_name: table_name.to_string(),
            _phantom: PhantomData,
        }
    }

    /// Helper method to prepare a statement
    pub async fn prepare_statement(&self, query: &str) -> DomainResponse<PreparedStatement> {
        self.session
            .prepare(query)
            .await
            .map_err(|e| Failure::DatabaseError(format!("Failed to prepare statement: {}", e)))
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
    async fn create(&self, entity: &E) -> DomainResponse<E> {
        let schema = S::from_entity(entity);
        let insert_columns = S::columns();
        let insert_placeholders = S::insert_placeholders();

        let query = format!(
            "INSERT INTO {}.{} ({}) VALUES ({})",
            self.keyspace, self.table_name, insert_columns, insert_placeholders
        );

        println!("Insert query: {}", query);
        let prepared = self.prepare_statement(&query).await?;

        let result = self
            .session
            .execute_unpaged(&prepared, &schema)
            .await
            .map_err(|e| Failure::DatabaseError(format!("Failed to create entity: {}", e)))?;

        println!("Insert result: {:?}", result);

        // Return the entity with the schema converted back
        Ok(schema.to_entity())
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
        let query = format!(
            "SELECT {} FROM {}.{} WHERE deleted_at = null ALLOW FILTERING",
            S::columns(),
            self.keyspace,
            self.table_name,
        );

        let prepared = self.prepare_statement(&query).await?;
        let entities = self
            .session
            .execute_unpaged(&prepared, ())
            .await
            .map_err(|e| Failure::DatabaseError(format!("Failed to find entities: {}", e)))?
            .into_rows_result();

        println!("Select result: {:?}", entities);
        Ok(vec![])
    }

    async fn finds_paginated(&self, _page: u32, _page_size: u32) -> DomainResponse<(Paginate, Vec<E>)> {
        todo!()
    }
}
