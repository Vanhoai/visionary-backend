use crate::{
    entities::session_entity::SessionEntity, repositories::session_repository::SessionRepository,
    usecases::session_usecases::FindSessionsQuery,
};
use async_trait::async_trait;
use shared::types::DomainResponse;
use std::sync::Arc;

#[async_trait]
pub trait SessionService: Send + Sync {
    async fn create_session(
        &self,
        account_id: &str,
        jit: &str,
        refresh_token: &str,
        expires_at: i64,
        ip_address: &str,
        user_agent: &str,
        device_type: &str,
    ) -> DomainResponse<SessionEntity>;
    async fn find_by_jit(&self, jit: &str) -> DomainResponse<Option<SessionEntity>>;
    async fn invalidate_session(&self, session_id: &str) -> DomainResponse<()>;
    async fn clean_session_by_account_id(&self, account_id: &str) -> DomainResponse<()>;
    async fn find_sessions(&self, query: &FindSessionsQuery) -> DomainResponse<Vec<SessionEntity>>;
}

pub struct SessionServiceImpl {
    repository: Arc<dyn SessionRepository>,
}

impl SessionServiceImpl {
    pub fn new(repository: Arc<dyn SessionRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl SessionService for SessionServiceImpl {
    async fn create_session(
        &self,
        account_id: &str,
        jit: &str,
        refresh_token: &str,
        expires_at: i64,
        ip_address: &str,
        user_agent: &str,
        device_type: &str,
    ) -> DomainResponse<SessionEntity> {
        let session = SessionEntity::new(
            false,
            account_id.to_string(),
            refresh_token.to_string(),
            jit.to_string(),
            expires_at,
            ip_address.to_string(),
            user_agent.to_string(),
            device_type.to_string(),
        );

        self.repository.create(&session).await
    }

    async fn find_by_jit(&self, jit: &str) -> DomainResponse<Option<SessionEntity>> {
        self.repository.find_by_jit(jit).await
    }

    async fn invalidate_session(&self, session_id: &str) -> DomainResponse<()> {
        self.repository.delete(session_id).await?;
        Ok(())
    }

    async fn clean_session_by_account_id(&self, account_id: &str) -> DomainResponse<()> {
        self.repository.remove_by_account_id(account_id).await
    }

    async fn find_sessions(&self, _query: &FindSessionsQuery) -> DomainResponse<Vec<SessionEntity>> {
        self.repository.finds().await
    }
}
