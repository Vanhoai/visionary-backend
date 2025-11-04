use crate::shared::models::{failure::HttpFailure, response::HttpResponse};

pub type AxumResponse<T> = Result<HttpResponse<T>, HttpFailure>;
