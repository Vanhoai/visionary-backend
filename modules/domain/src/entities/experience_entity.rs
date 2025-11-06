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
#[serde(rename_all = "camelCase")]
pub struct ExperienceEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub account_id: String,
    pub technologies: Vec<String>,
    pub position: String,
    pub responsibility: Vec<String>,
    pub company: String,
    pub location: String,
    pub start_date: i64,
    pub end_date: Option<i64>,
    pub is_current: bool,
}

impl ExperienceEntity {
    pub fn new(
        include_id: bool,
        account_id: &str,
        technologies: &Vec<String>,
        position: &str,
        responsibility: &Vec<String>,
        company: &str,
        location: &str,
        start_date: i64,
        end_date: Option<i64>,
        is_current: bool,
    ) -> DomainResponse<Self> {
        Self::validate_technologies(technologies.clone())?;
        Self::validate_position(position)?;
        Self::validate_responsibility(responsibility.clone())?;
        Self::validate_company(company)?;
        Self::validate_location(location)?;
        Self::validate_dates(start_date, end_date)?;

        Ok(ExperienceEntity {
            base: BaseEntity::new(include_id),
            account_id: account_id.to_string(),
            technologies: technologies.clone(),
            position: position.to_string(),
            responsibility: responsibility.clone(),
            company: company.to_string(),
            location: location.to_string(),
            start_date,
            end_date,
            is_current,
        })
    }

    fn validate_technologies(technologies: Vec<String>) -> DomainResponse<()> {
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

    fn validate_responsibility(responsibility: Vec<String>) -> DomainResponse<()> {
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

    fn validate_dates(start_date: i64, end_date: Option<i64>) -> DomainResponse<()> {
        if let Some(end) = end_date {
            if end < start_date {
                return Err(Failure::ValidationError("End date cannot be earlier than start date".to_string()));
            }
        }

        let now = chrono::Utc::now().timestamp_millis();
        if end_date.unwrap_or(now) > now {
            return Err(Failure::ValidationError("Dates cannot be in the future".to_string()));
        }

        Ok(())
    }
}
