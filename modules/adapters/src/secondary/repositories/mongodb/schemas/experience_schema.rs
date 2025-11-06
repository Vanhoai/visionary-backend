use crate::secondary::repositories::mongodb::mongo_base_repository::EntitySchema;
use crate::secondary::repositories::mongodb::schemas::base_schema::BaseSchema;
use domain::entities::experience_entity::ExperienceEntity;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExperienceSchema {
    #[serde(flatten)]
    pub base: BaseSchema,
    pub account_id: ObjectId,
    pub technologies: Vec<String>,
    pub position: String,
    pub responsibility: Vec<String>,
    pub company: String,
    pub location: String,
    pub start_date: i64,
    pub end_date: Option<i64>,
    pub is_current: bool,
}

impl EntitySchema<ExperienceEntity> for ExperienceSchema {
    fn from_entity(entity: ExperienceEntity) -> Self {
        ExperienceSchema {
            base: BaseSchema::from_entity(entity.base),
            account_id: ObjectId::parse_str(&entity.account_id).unwrap(),
            technologies: entity.technologies,
            position: entity.position,
            responsibility: entity.responsibility,
            company: entity.company,
            location: entity.location,
            start_date: entity.start_date,
            end_date: entity.end_date,
            is_current: entity.is_current,
        }
    }

    fn to_entity(&self) -> ExperienceEntity {
        ExperienceEntity {
            base: self.base.to_entity(),
            account_id: self.account_id.to_hex(),
            technologies: self.technologies.clone(),
            position: self.position.clone(),
            responsibility: self.responsibility.clone(),
            company: self.company.clone(),
            location: self.location.clone(),
            start_date: self.start_date,
            end_date: self.end_date,
            is_current: self.is_current,
        }
    }
}
