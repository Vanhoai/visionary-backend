use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::usecases::global_usecases::ManageExperienceUseCase;

// internal modules
use crate::shared::{
    di::state::AppState,
    models::{failure::HttpFailure, response::HttpResponse},
    types::AxumResponse,
};

pub async fn execute(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> AxumResponse<()> {
    match state.global_app_service.remove_experience_with_id(&id).await {
        Ok(_) => Ok(HttpResponse::new(StatusCode::OK, "Removed experience sucessfully 🐳", ())),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
