use axum::extract::{Query, State};
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::entities::session_entity::SessionEntity;
use domain::usecases::session_usecases::{FindSessionsQuery, ManageSessionUseCase};

// internal modules
use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    Query(query): Query<FindSessionsQuery>,
) -> AxumResponse<Vec<SessionEntity>> {
    match state.session_app_service.find_sessions(&query).await {
        Ok(response) => Ok(HttpResponse::new(StatusCode::OK, "Find sessions successful ⏰", response)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
