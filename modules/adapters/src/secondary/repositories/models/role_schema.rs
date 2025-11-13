use mongodb::bson::oid::ObjectId;
use scylla::SerializeRow;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// shared modules
use domain::entities::{base_entity::BaseEntity, role_entity::RoleEntity};

// internal modules
use crate::secondary::repositories::{
    models::base_schema::MongoBaseSchema, mongodb::mongo_base_repository, scylla::scylla_base_repository,
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

#[derive(Debug, Clone, SerializeRow)]
pub struct ScyllaRoleSchema {
    pub id: Option<Uuid>,
    pub account_id: String,
    pub role_name: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl scylla_base_repository::EntitySchema<RoleEntity> for ScyllaRoleSchema {
    fn from_entity(entity: &RoleEntity) -> Self {
        ScyllaRoleSchema {
            id: entity.base.id.as_ref().and_then(|id| Uuid::parse_str(id).ok()),
            created_at: entity.base.created_at,
            updated_at: entity.base.updated_at,
            deleted_at: entity.base.deleted_at,
            account_id: entity.account_id.clone(),
            role_name: entity.role_name.clone(),
        }
    }

    fn to_entity(&self) -> RoleEntity {
        RoleEntity {
            base: BaseEntity {
                id: self.id.as_ref().map(|id| id.to_string()),
                created_at: self.created_at,
                updated_at: self.updated_at,
                deleted_at: self.deleted_at,
            },
            account_id: self.account_id.clone(),
            role_name: self.role_name.clone(),
        }
    }

    fn columns() -> &'static str {
        "id, account_id, role_name, created_at, updated_at, deleted_at"
    }

    fn insert_placeholders() -> &'static str {
        "?, ?, ?, ?, ?, ?"
    }
}
