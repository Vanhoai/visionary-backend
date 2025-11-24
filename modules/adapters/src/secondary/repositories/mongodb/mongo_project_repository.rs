use async_trait::async_trait;
use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId},
};
use std::sync::Arc;

// shared modules
use domain::{entities::project_entity::ProjectEntity, repositories::project_repository::ProjectRepository};
use processors::MongoRepository;
use shared::{models::failure::Failure, types::DomainResponse};

// internal modules
use crate::secondary::repositories::{
    models::project_schema::MongoProjectSchema,
    mongodb::mongo_base_repository::{EntitySchema, MongoBaseRepository},
};

#[derive(MongoRepository)]
pub struct MongoProjectRepository {
    base: MongoBaseRepository<ProjectEntity, MongoProjectSchema>,
}

impl MongoProjectRepository {
    pub fn new(collection: Arc<Collection<MongoProjectSchema>>) -> Self {
        MongoProjectRepository { base: MongoBaseRepository::new(collection) }
    }
}

#[async_trait]
impl ProjectRepository for MongoProjectRepository {
    async fn find_by_name(&self, name: &str) -> DomainResponse<Option<ProjectEntity>> {
        let filter = doc! {
            "name": name,
            "deleted_at": { "$exists": false }
        };

        match self.base.collection.find_one(filter).await {
            Ok(Some(schema)) => Ok(Some(schema.to_entity())),
            Ok(None) => Ok(None),
            Err(e) => Err(Failure::DatabaseError(format!("Failed to find project by name: {}", e))),
        }
    }

    async fn update_project_partial(
        &self,
        id: &str,
        cover: Option<String>,
        name: Option<String>,
        description: Option<String>,
        link: Option<String>,
        github: Option<String>,
        tags: Option<Vec<String>>,
        markdown: Option<String>,
    ) -> DomainResponse<ProjectEntity> {
        let object_id =
            ObjectId::parse_str(id).map_err(|e| Failure::BadRequest(format!("Invalid id format: {}: {}", id, e)))?;

        // Build update document dynamically based on provided fields
        let mut update_doc = doc! {};

        if let Some(cover) = cover {
            update_doc.insert("cover", cover);
        }
        if let Some(name) = name {
            update_doc.insert("name", name);
        }
        if let Some(description) = description {
            update_doc.insert("description", description);
        }
        if let Some(link) = link {
            update_doc.insert("link", link);
        }
        if let Some(github) = github {
            update_doc.insert("github", github);
        }
        if let Some(tags) = tags {
            update_doc.insert("tags", tags);
        }
        if let Some(markdown) = markdown {
            update_doc.insert("markdown", markdown);
        }

        if update_doc.is_empty() {
            return Err(Failure::BadRequest("No fields provided for update".to_string()));
        }

        let filter = doc! { "_id": object_id };
        let update = doc! { "$set": update_doc };

        match self.base.collection.find_one_and_update(filter, update).await {
            Ok(Some(updated_schema)) => Ok(updated_schema.to_entity()),
            Ok(None) => Err(Failure::NotFound(format!("Project with id {} not found", id))),
            Err(e) => Err(Failure::DatabaseError(format!("Failed to update project: {}", e))),
        }
    }
}
