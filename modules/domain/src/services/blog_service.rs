use async_trait::async_trait;

#[async_trait]
pub trait BlogService: Send + Sync {}

pub struct BlogServiceImpl {}
