use async_trait::async_trait;
use serde::Deserialize;
use shared::types::DomainResponse;

use crate::entities::session_entity::SessionEntity;

#[derive(Debug, Deserialize)]
pub struct FindSessionsQuery {}

#[async_trait]
pub trait ManageSessionUseCase: Send + Sync {
    async fn find_sessions(&self, query: &FindSessionsQuery) -> DomainResponse<Vec<SessionEntity>>;
    async fn invalidate_session(&self, session_id: &str) -> DomainResponse<()>;
}
