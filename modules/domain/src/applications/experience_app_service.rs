use crate::services::experience_service::ExperienceService;
use std::sync::Arc;

pub struct ExperienceAppService {
    experience_service: Arc<dyn ExperienceService>,
}

impl ExperienceAppService {
    pub fn new(experience_service: Arc<dyn ExperienceService>) -> Self {
        Self { experience_service }
    }
}