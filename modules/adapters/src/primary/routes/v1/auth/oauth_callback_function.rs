use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;
use crate::shared::utilities::request_extractor;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use domain::usecases::auth_usecases::{AuthResponse, OAuth2CallbackParams, OAuth2UseCase, SessionMetadata};
use shared::models::failure::Failure;
use std::sync::Arc;
use validator::Validate;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<OAuth2CallbackParams>,
) -> AxumResponse<AuthResponse> {
    params.validate().map_err(|e| HttpFailure::new(Failure::ValidationError(e.to_string())))?;

    let ip_address = request_extractor::extract_ip(&headers);
    let user_agent = request_extractor::extract_user_agent(&headers);
    let device_type = request_extractor::detect_device_type(&user_agent);
    let session_metadata = SessionMetadata { ip_address, user_agent, device_type };

    match state.auth_app_service.oauth2_callback(&params, &session_metadata).await {
        Ok(response) => Ok(HttpResponse::new(StatusCode::OK, "OAuth2 authentication successful ✅", response)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
