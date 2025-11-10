use async_trait::async_trait;
use domain::entities::account_entity::AccountEntity;
use domain::repositories::account_repository::AccountRepository;
use domain::repositories::base_repository::BaseRepository;
use shared::models::paginate::Paginate;
use shared::types::DomainResponse;

pub struct ScyllaAccountRepository {}

impl ScyllaAccountRepository {
    pub fn new() -> Self {
        ScyllaAccountRepository {}
    }
}

#[async_trait]
impl BaseRepository<AccountEntity> for ScyllaAccountRepository {
    async fn create(&self, _entity: &AccountEntity) -> DomainResponse<AccountEntity> {
        todo!()
    }

    async fn update(&self, _id: &str, _entity: &AccountEntity) -> DomainResponse<AccountEntity> {
        todo!()
    }

    async fn delete(&self, _: &str) -> DomainResponse<AccountEntity> {
        todo!()
    }

    async fn remove(&self, _: &str) -> DomainResponse<AccountEntity> {
        todo!()
    }

    async fn find(&self, _: &str) -> DomainResponse<Option<AccountEntity>> {
        todo!()
    }

    async fn find_and_delete(&self, _id: &str) -> DomainResponse<AccountEntity> {
        todo!()
    }

    async fn find_and_remove(&self, _id: &str) -> DomainResponse<AccountEntity> {
        todo!()
    }

    async fn finds(&self) -> DomainResponse<Vec<AccountEntity>> {
        todo!()
    }

    async fn finds_paginated(&self, _page: u32, _page_size: u32) -> DomainResponse<(Paginate, Vec<AccountEntity>)> {
        todo!()
    }
}

#[async_trait]
impl AccountRepository for ScyllaAccountRepository {
    async fn find_by_email(&self, _: &str) -> DomainResponse<Option<AccountEntity>> {
        todo!()
    }
}
