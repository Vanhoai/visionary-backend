use crate::entities::base_entity::BaseEntity;
use serde::{Deserialize, Serialize};
use shared::models::failure::Failure;
use shared::types::DomainResponse;

static MAX_TECHNOLOGIES_LENGTH: usize = 255;
static MAX_POSITION_LENGTH: usize = 100;
static MAX_RESPONSIBILITY_LENGTH: usize = 1000;
static MAX_COMPANY_LENGTH: usize = 100;
static MAX_LOCATION_LENGTH: usize = 100;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub account_id: String,
    pub technologies: String,
    pub position: String,
    pub responsibility: String,
    pub company: String,
    pub location: String,
}

impl WorkEntity {
    pub fn new(
        include_id: bool,
        account_id: String,
        technologies: String,
        position: String,
        responsibility: String,
        company: String,
        location: String,
    ) -> DomainResponse<Self> {
        Self::validate_technologies(&technologies)?;
        Self::validate_position(&position)?;
        Self::validate_responsibility(&responsibility)?;
        Self::validate_company(&company)?;
        Self::validate_location(&location)?;

        Ok(WorkEntity {
            base: BaseEntity::new(include_id),
            account_id,
            technologies,
            position,
            responsibility,
            company,
            location,
        })
    }

    fn validate_technologies(technologies: &str) -> DomainResponse<()> {
        if technologies.len() > MAX_TECHNOLOGIES_LENGTH {
            return Err(Failure::ValidationError(format!(
                "Technologies must not exceed {} characters",
                MAX_TECHNOLOGIES_LENGTH
            )));
        }

        Ok(())
    }

    fn validate_position(position: &str) -> DomainResponse<()> {
        if position.len() > MAX_POSITION_LENGTH {
            return Err(Failure::ValidationError(format!(
                "Position must not exceed {} characters",
                MAX_POSITION_LENGTH
            )));
        }

        Ok(())
    }

    fn validate_responsibility(responsibility: &str) -> DomainResponse<()> {
        if responsibility.len() > MAX_RESPONSIBILITY_LENGTH {
            return Err(Failure::ValidationError(format!(
                "Responsibility must not exceed {} characters",
                MAX_RESPONSIBILITY_LENGTH
            )));
        }

        Ok(())
    }

    fn validate_company(company: &str) -> DomainResponse<()> {
        if company.len() > MAX_COMPANY_LENGTH {
            return Err(Failure::ValidationError(format!("Company must not exceed {} characters", MAX_COMPANY_LENGTH)));
        }

        Ok(())
    }

    fn validate_location(location: &str) -> DomainResponse<()> {
        if location.len() > MAX_LOCATION_LENGTH {
            return Err(Failure::ValidationError(format!(
                "Location must not exceed {} characters",
                MAX_LOCATION_LENGTH
            )));
        }

        Ok(())
    }
}
