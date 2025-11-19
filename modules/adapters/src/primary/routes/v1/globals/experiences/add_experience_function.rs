use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::{
    entities::experience_entity::ExperienceEntity,
    usecases::global_usecases::{AddExperienceParams, ManageExperienceUseCase},
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
    ValidatedPayload(params): ValidatedPayload<AddExperienceParams>,
) -> AxumResponse<ExperienceEntity> {
    match state.global_app_service.add_experience(&params).await {
        Ok(experience) => Ok(HttpResponse::new(StatusCode::OK, "Experience added successfully ✅", experience)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
