use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// shared modules
use domain::entities::role_entity::RoleEntity;

// internal modules
use crate::secondary::repositories::{
    models::base_schema::{MongoBaseSchema, ScyllaBaseSchema},
    mongodb::mongo_base_repository,
    scylla::scylla_base_repository,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoRoleSchema {
    #[serde(flatten)]
    base: MongoBaseSchema,
    pub account_id: ObjectId,
    pub role_name: String,
}

impl mongo_base_repository::EntitySchema<RoleEntity> for MongoRoleSchema {
    fn from_entity(entity: &RoleEntity) -> Self {
        MongoRoleSchema {
            base: MongoBaseSchema::from_entity(&entity.base),
            account_id: ObjectId::parse_str(&entity.account_id).unwrap(),
            role_name: entity.role_name.clone(),
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

#[derive(Debug, Clone)]
pub struct ScyllaRoleSchema {
    pub base: ScyllaBaseSchema,
    pub account_id: String,
    pub role_name: String,
}

impl scylla_base_repository::EntitySchema<RoleEntity> for ScyllaRoleSchema {
    fn from_entity(entity: &RoleEntity) -> Self {
        ScyllaRoleSchema {
            base: ScyllaBaseSchema::from_entity(&entity.base),
            account_id: entity.account_id.clone(),
            role_name: entity.role_name.clone(),
        }
    }

    fn to_entity(&self) -> RoleEntity {
        RoleEntity {
            base: self.base.to_entity(),
            account_id: self.account_id.clone(),
            role_name: self.role_name.clone(),
        }
    }
}
