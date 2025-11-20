use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::{
    entities::project_entity::ProjectEntity,
    usecases::global_usecases::{AddProjectParams, ManageProjectUseCase},
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
    ValidatedPayload(params): ValidatedPayload<AddProjectParams>,
) -> AxumResponse<ProjectEntity> {
    match state.global_app_service.add_project(&params).await {
        Ok(project) => Ok(HttpResponse::new(StatusCode::CREATED, "Add new project sucessfully 🐳", project)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
