use mongodb::bson::{Document, doc};
use std::fmt::Debug;

pub trait DatabaseFilter: Debug + Clone + Send + Sync {}

#[derive(Debug, Clone)]
pub struct MongoFilter {
    pub criteria: Vec<FilterCriteria>,
}

#[derive(Debug, Clone)]
pub struct FilterCriteria {
    pub field: String,
    pub operator: FilterOperator,
    pub value: FilterValue,
}

#[derive(Debug, Clone)]
pub enum FilterOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    NotIn,
    Contains,
    StartsWith,
    EndsWith,
}

#[derive(Debug, Clone)]
pub enum FilterValue {
    String(String),
    Number(i64),
    Boolean(bool),
    StringArray(Vec<String>),
    NumberArray(Vec<i64>),
}

impl DatabaseFilter for MongoFilter {}

#[derive(Debug, Clone)]
pub struct ScyllaFilter {
    pub partition_keys: Vec<(String, ScyllaValue)>,
    pub clustering_conditions: Vec<ClusteringCondition>,
}

impl DatabaseFilter for ScyllaFilter {}

#[derive(Debug, Clone)]
pub enum ScyllaValue {
    String(String),
    Int(i32),
    BigInt(i64),
    Boolean(bool),
    Uuid(uuid::Uuid),
}

#[derive(Debug, Clone)]
pub struct ClusteringCondition {
    pub column: String,
    pub operator: ScyllaOperator,
    pub value: ScyllaValue,
}

#[derive(Debug, Clone)]
pub enum ScyllaOperator {
    Eq,
    Gt,
    Gte,
    Lt,
    Lte,
}

pub struct MongoFilterConverter;

impl MongoFilterConverter {
    pub fn convert_to_mongo_filter(query: &MongoFilter) -> Document {
        let mut filter = doc! {
            "deleted_at": { "$exists": false }
        };

        for criteria in &query.criteria {
            let mongo_criteria = Self::convert_criteria(criteria);
            filter.insert(&criteria.field, mongo_criteria);
        }

        filter
    }

    fn convert_criteria(criteria: &FilterCriteria) -> mongodb::bson::Bson {
        match (&criteria.operator, &criteria.value) {
            (FilterOperator::Eq, FilterValue::String(v)) => doc! { "$eq": v }.into(),
            (FilterOperator::Eq, FilterValue::Number(v)) => doc! { "$eq": v }.into(),
            (FilterOperator::Eq, FilterValue::Boolean(v)) => doc! { "$eq": v }.into(),

            (FilterOperator::Ne, FilterValue::String(v)) => doc! { "$ne": v }.into(),
            (FilterOperator::Ne, FilterValue::Number(v)) => doc! { "$ne": v }.into(),

            (FilterOperator::Gt, FilterValue::Number(v)) => doc! { "$gt": v }.into(),
            (FilterOperator::Gte, FilterValue::Number(v)) => doc! { "$gte": v }.into(),
            (FilterOperator::Lt, FilterValue::Number(v)) => doc! { "$lt": v }.into(),
            (FilterOperator::Lte, FilterValue::Number(v)) => doc! { "$lte": v }.into(),

            (FilterOperator::In, FilterValue::StringArray(v)) => doc! { "$in": v }.into(),
            (FilterOperator::In, FilterValue::NumberArray(v)) => doc! { "$in": v }.into(),

            (FilterOperator::NotIn, FilterValue::StringArray(v)) => doc! { "$nin": v }.into(),
            (FilterOperator::NotIn, FilterValue::NumberArray(v)) => doc! { "$nin": v }.into(),

            (FilterOperator::Contains, FilterValue::String(v)) => doc! { "$regex": v, "$options": "i" }.into(),

            (FilterOperator::StartsWith, FilterValue::String(v)) => {
                doc! { "$regex": format!("^{}", v), "$options": "i" }.into()
            },

            (FilterOperator::EndsWith, FilterValue::String(v)) => {
                doc! { "$regex": format!("{}$", v), "$options": "i" }.into()
            },

            _ => doc! {}.into(),
        }
    }
}
