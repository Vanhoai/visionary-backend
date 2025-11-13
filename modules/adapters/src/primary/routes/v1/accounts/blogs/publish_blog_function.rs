use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::{
    entities::blog_entity::BlogEntity,
    usecases::account_usecases::{MangeBlogAccountUseCase, PublishBlogParams},
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
    Path(id): Path<String>,
    ValidatedPayload(params): ValidatedPayload<PublishBlogParams>,
) -> AxumResponse<BlogEntity> {
    match state.account_app_service.publish_account_blog(&id, &params).await {
        Ok(blog_entity) => Ok(HttpResponse::new(StatusCode::CREATED, "Blog published successfully", blog_entity)),
        Err(err) => Err(HttpFailure::new(err)),
    }
}
