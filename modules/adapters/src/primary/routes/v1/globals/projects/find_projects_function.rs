use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::{entities::project_entity::ProjectEntity, usecases::global_usecases::ManageProjectUseCase};

// internal modules
use crate::shared::{
    di::state::AppState,
    models::{failure::HttpFailure, response::HttpResponse},
    types::AxumResponse,
};

pub async fn execute(State(state): State<Arc<AppState>>) -> AxumResponse<Vec<ProjectEntity>> {
    match state.global_app_service.find_projects().await {
        Ok(project) => Ok(HttpResponse::new(StatusCode::OK, "Retrieved projects sucessfully 🐳", project)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
