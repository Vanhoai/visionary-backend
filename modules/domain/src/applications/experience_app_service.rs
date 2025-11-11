use std::sync::Arc;

// internal modules
use crate::services::experience_service::ExperienceService;

#[allow(dead_code)]
pub struct ExperienceAppService {
    experience_service: Arc<dyn ExperienceService>,
}

impl ExperienceAppService {
    pub fn new(experience_service: Arc<dyn ExperienceService>) -> Self {
        Self { experience_service }
    }
}
