use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::usecases::account_usecases::MangeBlogAccountUseCase;

// internal modules
use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::{models::response::HttpResponse, types::AxumResponse};

pub async fn execute(
    State(state): State<Arc<AppState>>,
    Path((account_id, blog_id)): Path<(String, String)>,
) -> AxumResponse<()> {
    match state.account_app_service.delete_account_blog(&account_id, &blog_id).await {
        Ok(_) => Ok(HttpResponse::new(StatusCode::OK, "Blog deleted successfully", ())),
        Err(err) => Err(HttpFailure::new(err)),
    }
}
