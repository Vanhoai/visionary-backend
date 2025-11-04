use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    pub id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}

impl BaseEntity {
    pub fn new(include_id: bool) -> Self {
        let uuid = Uuid::now_v7().to_string();
        let now = chrono::Utc::now().timestamp();
        let id = if include_id { Some(uuid.clone()) } else { None };

        BaseEntity { id, created_at: now, updated_at: now, deleted_at: None }
    }
}
