use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[macro_export]
macro_rules! define_update_struct {
    (
        $vis:vis struct $update_name:ident for $original_name:ident {
            $($field_vis:vis $field_name:ident : $field_type:ty),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        $vis struct $update_name {
            $(
                $field_vis $field_name: Option<$field_type>,
            )*
        }
    };
}
