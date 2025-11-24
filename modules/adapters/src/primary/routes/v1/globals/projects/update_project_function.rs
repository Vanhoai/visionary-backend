use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::entities::project_entity::ProjectEntity;
use domain::usecases::global_usecases::{ManageProjectUseCase, UpdateProjectParams};

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
    ValidatedPayload(params): ValidatedPayload<UpdateProjectParams>,
) -> AxumResponse<ProjectEntity> {
    match state.global_app_service.update_project(&id, &params).await {
        Ok(experience) => Ok(HttpResponse::new(StatusCode::OK, "Updated project successfully 🐳", experience)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
