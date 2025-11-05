use crate::functions::deserialize_functions::deserialize_number_from_string;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paginate {
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub total_records: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BasePaginateQuery {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[validate(range(min = 1, max = 100))]
    pub page: u32,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[validate(range(min = 1, max = 100))]
    pub page_size: u32,
}
