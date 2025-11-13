use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::{
    entities::blog_entity::BlogEntity,
    usecases::account_usecases::{MangeBlogAccountUseCase, UpdateBlogParams},
};

// internal modules
use crate::shared::{
    di::state::AppState,
    models::{failure::HttpFailure, response::HttpResponse},
    types::AxumResponse,
    utilities::validated_payload::ValidatedPayload,
};

pub async fn execute(
    State(state): State<Arc<AppState>>,
    Path((account_id, blog_id)): Path<(String, String)>,
    ValidatedPayload(params): ValidatedPayload<UpdateBlogParams>,
) -> AxumResponse<BlogEntity> {
    match state.account_app_service.update_account_blog(&account_id, &blog_id, &params).await {
        Ok(blog_entity) => Ok(HttpResponse::new(StatusCode::CREATED, "Updated blog successfully 🪼", blog_entity)),
        Err(err) => Err(HttpFailure::new(err)),
    }
}
