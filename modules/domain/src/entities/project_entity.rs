use serde::{Deserialize, Serialize};
use shared::{models::failure::Failure, types::DomainResponse};

// internal modules
use crate::entities::base_entity::BaseEntity;

static MAX_LENGTH_NAME: usize = 100;
static MAX_LENGTH_DESCRIPTION: usize = 2 * 1024; // 2 KB

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectEntity {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub cover: String,
    pub name: String,
    pub description: String,
    pub link: String,
    pub github: String,
    pub tags: Vec<String>,
    pub markdown: String,
}

crate::define_update_struct! {
    pub struct UpdateProjectEntity for ProjectEntity {
        pub cover: String,
        pub name: String,
        pub description: String,
        pub link: String,
        pub github: String,
        pub tags: Vec<String>,
        pub markdown: String,
    }
}

impl ProjectEntity {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        include_id: bool,
        cover: &str,
        name: &str,
        description: &str,
        link: &str,
        github: &str,
        tags: &[String],
        markdown: &str,
    ) -> Self {
        Self {
            base: BaseEntity::new(include_id),
            cover: cover.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            link: link.to_string(),
            github: github.to_string(),
            tags: tags.to_owned(),
            markdown: markdown.to_string(),
        }
    }

    pub fn validate_name(name: &str) -> DomainResponse<()> {
        if name.is_empty() {
            return Err(Failure::ValidationError("Project name must not be empty".to_string()));
        }

        if name.len() > MAX_LENGTH_NAME {
            return Err(Failure::ValidationError(format!(
                "Project name must not exceed {} characters",
                MAX_LENGTH_NAME
            )));
        }

        Ok(())
    }

    pub fn validate_description(description: &str) -> DomainResponse<()> {
        if description.is_empty() {
            return Err(Failure::ValidationError("Project description must not be empty".to_string()));
        }

        if description.len() > MAX_LENGTH_DESCRIPTION {
            return Err(Failure::ValidationError(format!(
                "Project description must not exceed {} characters",
                MAX_LENGTH_DESCRIPTION
            )));
        }

        Ok(())
    }
}
