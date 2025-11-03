use std::sync::Arc;
use async_trait::async_trait;
use crate::repositories::session_repository::SessionRepository;

#[async_trait]
pub trait SessionService: Send + Sync {}

pub struct SessionServiceImpl {
    repository: Arc<dyn SessionRepository>
}

impl SessionServiceImpl {
    pub fn new(repository: Arc<dyn SessionRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl SessionService for SessionServiceImpl {}
