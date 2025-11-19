use scylla::SerializeRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// shared modules
use domain::entities::{base_entity::BaseEntity, experience_entity::ExperienceEntity};

// internal modules
use crate::secondary::repositories::{
    models::base_schema::MongoBaseSchema, mongodb::mongo_base_repository::EntitySchema, scylla::scylla_base_repository,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MongoExperienceSchema {
    #[serde(flatten)]
    pub base: MongoBaseSchema,
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

#[derive(Debug, Clone, SerializeRow)]
pub struct ScyllaExperienceSchema {
    pub id: Option<Uuid>,
    pub technologies: Vec<String>,
    pub position: String,
    pub responsibility: Vec<String>,
    pub company: String,
    pub location: String,
    pub start_date: i64,
    pub end_date: Option<i64>,
    pub is_current: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl scylla_base_repository::EntitySchema<ExperienceEntity> for ScyllaExperienceSchema {
    fn from_entity(entity: &ExperienceEntity) -> Self {
        ScyllaExperienceSchema {
            id: entity.base.id.as_ref().and_then(|id| Uuid::parse_str(id).ok()),
            technologies: entity.technologies.clone(),
            position: entity.position.clone(),
            responsibility: entity.responsibility.clone(),
            company: entity.company.clone(),
            location: entity.location.clone(),
            start_date: entity.start_date,
            end_date: entity.end_date,
            is_current: entity.is_current,
            created_at: entity.base.created_at,
            updated_at: entity.base.updated_at,
            deleted_at: entity.base.deleted_at,
        }
    }

    fn to_entity(&self) -> ExperienceEntity {
        ExperienceEntity {
            base: BaseEntity {
                id: self.id.as_ref().map(|id| id.to_string()),
                created_at: self.created_at,
                updated_at: self.updated_at,
                deleted_at: self.deleted_at,
            },
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

    fn columns() -> &'static str {
        "id, technologies, position, responsibility, company, location, start_date, end_date, is_current, created_at, updated_at, deleted_at"
    }

    fn insert_placeholders() -> &'static str {
        "?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?"
    }
}
