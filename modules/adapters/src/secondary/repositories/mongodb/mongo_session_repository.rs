use async_trait::async_trait;
use domain::repositories::session_repository::SessionRepository;

pub struct MongoSessionRepository {}

impl MongoSessionRepository {
    pub fn new() -> Self {
        MongoSessionRepository {}
    }
}

#[async_trait]
impl SessionRepository for MongoSessionRepository {}
