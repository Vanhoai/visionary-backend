use async_trait::async_trait;

// shared modules
use shared::types::DomainResponse;

// internal modules
use crate::entities::session_entity::SessionEntity;
use crate::repositories::base_repository::BaseRepository;

#[async_trait]
pub trait SessionRepository: BaseRepository<SessionEntity> {
    async fn remove_by_account_id(&self, account_id: &str) -> DomainResponse<()>;
    async fn find_by_jti(&self, jti: &str) -> DomainResponse<Option<SessionEntity>>;
}
