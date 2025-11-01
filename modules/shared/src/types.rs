use crate::models::failure::Failure;

pub type DomainResponse<T> = Result<T, Failure>;
