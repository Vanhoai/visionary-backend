use crate::secondary::repositories::mongodb::mongo_base_repository::EntitySchema;
use crate::secondary::repositories::mongodb::schemas::base_schema::BaseSchema;
use domain::entities::work_entity::WorkEntity;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkSchema {
    #[serde(flatten)]
    pub base: BaseSchema,
    pub account_id: ObjectId,
    pub technologies: String,
    pub position: String,
    pub responsibility: String,
    pub company: String,
    pub location: String,
}

impl EntitySchema<WorkEntity> for WorkSchema {
    fn from_entity(entity: WorkEntity) -> Self {
        WorkSchema {
            base: BaseSchema::from_entity(entity.base),
            account_id: ObjectId::parse_str(&entity.account_id).unwrap(),
            technologies: entity.technologies,
            position: entity.position,
            responsibility: entity.responsibility,
            company: entity.company,
            location: entity.location,
        }
    }

    fn to_entity(&self) -> WorkEntity {
        WorkEntity {
            base: self.base.to_entity(),
            account_id: self.account_id.to_hex(),
            technologies: self.technologies.clone(),
            position: self.position.clone(),
            responsibility: self.responsibility.clone(),
            company: self.company.clone(),
            location: self.location.clone(),
        }
    }
}
