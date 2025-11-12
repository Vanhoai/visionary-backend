use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// shared modules
use domain::entities::experience_entity::ExperienceEntity;
use uuid::Uuid;

// internal modules
use crate::secondary::repositories::{
    models::base_schema::{MongoBaseSchema, ScyllaBaseSchema},
    mongodb::mongo_base_repository::EntitySchema,
    scylla::scylla_base_repository,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MongoExperienceSchema {
    #[serde(flatten)]
    pub base: MongoBaseSchema,
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

impl EntitySchema<ExperienceEntity> for MongoExperienceSchema {
    fn from_entity(entity: &ExperienceEntity) -> Self {
        MongoExperienceSchema {
            base: MongoBaseSchema::from_entity(&entity.base),
            account_id: ObjectId::parse_str(&entity.account_id).unwrap(),
            technologies: entity.technologies.clone(),
            position: entity.position.clone(),
            responsibility: entity.responsibility.clone(),
            company: entity.company.clone(),
            location: entity.location.clone(),
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

#[derive(Debug, Clone)]
pub struct ScyllaExperienceSchema {
    pub base: ScyllaBaseSchema,
    pub account_id: Uuid,
    pub technologies: Vec<String>,
    pub position: String,
    pub responsibility: Vec<String>,
    pub company: String,
    pub location: String,
    pub start_date: i64,
    pub end_date: Option<i64>,
    pub is_current: bool,
}

impl scylla_base_repository::EntitySchema<ExperienceEntity> for ScyllaExperienceSchema {
    fn from_entity(entity: &ExperienceEntity) -> Self {
        ScyllaExperienceSchema {
            base: ScyllaBaseSchema::from_entity(&entity.base),
            account_id: Uuid::parse_str(&entity.account_id).unwrap(),
            technologies: entity.technologies.clone(),
            position: entity.position.clone(),
            responsibility: entity.responsibility.clone(),
            company: entity.company.clone(),
            location: entity.location.clone(),
            start_date: entity.start_date,
            end_date: entity.end_date,
            is_current: entity.is_current,
        }
    }

    fn to_entity(&self) -> ExperienceEntity {
        ExperienceEntity {
            base: self.base.to_entity(),
            account_id: self.account_id.to_string(),
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
