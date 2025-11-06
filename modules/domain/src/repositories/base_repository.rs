use async_trait::async_trait;
use shared::models::paginate::Paginate;
use shared::types::DomainResponse;

#[async_trait]
pub trait BaseRepository<E>: Send + Sync {
    async fn create(&self, entity: &E) -> DomainResponse<E>;
    async fn update(&self, id: &str, entity: &E) -> DomainResponse<E>;
    async fn delete(&self, id: &str) -> DomainResponse<E>;
    async fn remove(&self, id: &str) -> DomainResponse<E>;
    async fn find(&self, id: &str) -> DomainResponse<Option<E>>;
    async fn finds(&self) -> DomainResponse<Vec<E>>;
    async fn finds_paginated(&self, page: u32, page_size: u32) -> DomainResponse<(Paginate, Vec<E>)>;
}
