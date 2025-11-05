use crate::shared::models::{
    failure::HttpFailure,
    response::{HttpPaginatedResponse, HttpResponse},
};

pub type AxumResponse<T> = Result<HttpResponse<T>, HttpFailure>;
pub type AxumPaginatedResponse<T> = Result<HttpPaginatedResponse<T>, HttpFailure>;
