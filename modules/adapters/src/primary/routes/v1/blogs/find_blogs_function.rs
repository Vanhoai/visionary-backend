use axum::http::StatusCode;

// internal modules
use crate::shared::{models::response::HttpResponse, types::AxumResponse};

pub async fn execute() -> AxumResponse<()> {
    Ok(HttpResponse::new(StatusCode::OK, "Retrieved all blog successfully 🪼", ()))
}
