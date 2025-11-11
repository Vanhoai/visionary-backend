use axum::extract::State;
use reqwest::StatusCode;
use std::sync::Arc;

// shared modules
use domain::entities::category_entity::CategoryEntity;
use domain::usecases::category_usecases::{CreateCategoryParams, ManageCategoryUseCase};

// internal modules
use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;
use crate::shared::utilities::validated_payload::ValidatedPayload;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    ValidatedPayload(params): ValidatedPayload<CreateCategoryParams>,
) -> AxumResponse<CategoryEntity> {
    match state.category_app_service.create_category(&params).await {
        Ok(category) => Ok(HttpResponse::new(StatusCode::CREATED, "Category created successfully", category)),
        Err(err) => Err(HttpFailure::new(err)),
    }
}
