#[derive(Debug)]
pub struct CategoryEntity {
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,
}