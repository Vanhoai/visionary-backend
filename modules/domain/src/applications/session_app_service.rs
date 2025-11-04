use crate::{
    entities::session_entity::SessionEntity,
    services::session_service::SessionService,
    usecases::session_usecases::{FindSessionsQuery, ManageSessionUseCase},
};
use async_trait::async_trait;
use shared::types::DomainResponse;
use std::sync::Arc;

pub struct SessionAppService {
    session_service: Arc<dyn SessionService>,
}

impl SessionAppService {
    pub fn new(session_service: Arc<dyn SessionService>) -> Self {
        Self { session_service }
    }
}

// region =============================== MANAGE SESSION USE CASES ===============================
#[async_trait]
impl ManageSessionUseCase for SessionAppService {
    async fn find_sessions(&self, query: &FindSessionsQuery) -> DomainResponse<Vec<SessionEntity>> {
        self.session_service.find_sessions(query).await
    }

    async fn invalidate_session(&self, session_id: &str) -> DomainResponse<()> {
        self.session_service.invalidate_session(session_id).await
    }
}
// endregion =============================== MANAGE SESSION USE CASES ===============================
