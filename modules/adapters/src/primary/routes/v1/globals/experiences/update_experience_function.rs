use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::{
    entities::experience_entity::ExperienceEntity,
    usecases::global_usecases::{ManageExperienceUseCase, UpdateExperienceParams},
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
    ValidatedPayload(params): ValidatedPayload<UpdateExperienceParams>,
) -> AxumResponse<ExperienceEntity> {
    match state.global_app_service.update_experience(&id, &params).await {
        Ok(experience) => Ok(HttpResponse::new(StatusCode::OK, "Updated experience successfully 🐳", experience)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
