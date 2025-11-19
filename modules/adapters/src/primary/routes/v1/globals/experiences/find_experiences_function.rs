use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::{entities::experience_entity::ExperienceEntity, usecases::global_usecases::ManageExperienceUseCase};

// internal modules
use crate::shared::{
    di::state::AppState,
    models::{failure::HttpFailure, response::HttpResponse},
    types::AxumResponse,
};

pub async fn execute(State(state): State<Arc<AppState>>) -> AxumResponse<Vec<ExperienceEntity>> {
    match state.global_app_service.find_experiences().await {
        Ok(experiences) => Ok(HttpResponse::new(StatusCode::OK, "Experiences retrieved successfully ✅", experiences)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
