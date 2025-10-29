use async_trait::async_trait;
use domain::repositories::session_repository::SessionRepository;

pub struct SessionRepositoryImpl {}

impl SessionRepositoryImpl {
    pub fn new() -> Self {
        SessionRepositoryImpl {}
    }
}

#[async_trait]
impl SessionRepository for SessionRepositoryImpl {}
