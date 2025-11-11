use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::entities::category_entity::CategoryEntity;
use domain::usecases::category_usecases::ManageCategoryUseCase;

// internal modules
use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;

pub async fn execute(State(state): State<Arc<AppState>>) -> AxumResponse<Vec<CategoryEntity>> {
    match state.category_app_service.find_categories().await {
        Ok(categories) => Ok(HttpResponse::new(StatusCode::OK, "Categories retrieved successfully 🪼", categories)),
        Err(err) => Err(HttpFailure::new(err)),
    }
}
