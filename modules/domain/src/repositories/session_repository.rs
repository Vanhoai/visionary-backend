use crate::entities::session_entity::SessionEntity;
use crate::repositories::base_repository::BaseRepository;
use async_trait::async_trait;
use shared::types::DomainResponse;

#[async_trait]
pub trait SessionRepository: BaseRepository<SessionEntity> {
    async fn remove_by_account_id(&self, account_id: &str) -> DomainResponse<()>;
    async fn find_by_jit(&self, jit: &str) -> DomainResponse<Option<SessionEntity>>;
}
