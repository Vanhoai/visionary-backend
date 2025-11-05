use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;
use axum::extract::{Query, State};
use domain::usecases::auth_usecases::{OAuth2InitParams, OAuth2InitResponse, OAuth2UseCase};
use reqwest::StatusCode;
use std::sync::Arc;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    Query(query): Query<OAuth2InitParams>,
) -> AxumResponse<OAuth2InitResponse> {
    match state.auth_app_service.oauth2_init(&query).await {
        Ok(response) => Ok(HttpResponse::new(StatusCode::OK, "OAuth2 initialization successful ✅".to_string(), response)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
