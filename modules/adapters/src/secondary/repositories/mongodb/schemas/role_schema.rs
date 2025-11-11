use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// shared modules
use domain::entities::role_entity::RoleEntity;

// internal modules
use crate::secondary::repositories::mongodb::mongo_base_repository::EntitySchema;
use crate::secondary::repositories::mongodb::schemas::base_schema::BaseSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleSchema {
    #[serde(flatten)]
    base: BaseSchema,
    pub account_id: ObjectId,
    pub role_name: String,
}

impl EntitySchema<RoleEntity> for RoleSchema {
    fn from_entity(entity: RoleEntity) -> Self {
        RoleSchema {
            base: BaseSchema::from_entity(entity.base),
            account_id: ObjectId::parse_str(&entity.account_id).unwrap(),
            role_name: entity.role_name,
        }
    }

    fn to_entity(&self) -> RoleEntity {
        RoleEntity {
            base: self.base.to_entity(),
            account_id: self.account_id.to_hex(),
            role_name: self.role_name.clone(),
        }
    }
}
