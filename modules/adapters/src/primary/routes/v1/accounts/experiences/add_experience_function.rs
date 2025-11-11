use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::entities::experience_entity::ExperienceEntity;
use domain::usecases::account_usecases::{AddExperienceToAccountParams, ManageExperienceAccountUseCase};

// internal modules
use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;
use crate::shared::utilities::validated_payload::ValidatedPayload;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ValidatedPayload(params): ValidatedPayload<AddExperienceToAccountParams>,
) -> AxumResponse<ExperienceEntity> {
    match state.account_app_service.add_experience_to_account(&id, &params).await {
        Ok(experience) => {
            Ok(HttpResponse::new(StatusCode::CREATED, "Experience added to account successfully ✅", experience))
        },
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
