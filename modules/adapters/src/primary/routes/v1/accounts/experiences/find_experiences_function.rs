use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use domain::entities::experience_entity::ExperienceEntity;
use domain::usecases::account_usecases::ManageExperienceAccountUseCase;
use std::sync::Arc;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> AxumResponse<Vec<ExperienceEntity>> {
    match state.account_app_service.find_experiences_by_account_id(&id).await {
        Ok(response) => Ok(HttpResponse::new(StatusCode::OK, "Experiences retrieved successfully 🎨", response)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
