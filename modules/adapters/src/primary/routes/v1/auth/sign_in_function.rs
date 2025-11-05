use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;
use crate::shared::utilities::request_extractor;
use crate::shared::utilities::validated_payload::ValidatedPayload;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use domain::usecases::auth_usecases::{AuthParams, AuthResponse, ManageSessionAuthUseCase, SessionMetadata};
use std::sync::Arc;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    ValidatedPayload(params): ValidatedPayload<AuthParams>,
) -> AxumResponse<AuthResponse> {
    let ip_address = request_extractor::extract_ip(&headers);
    let user_agent = request_extractor::extract_user_agent(&headers);
    let device_type = request_extractor::detect_device_type(&user_agent);
    let session_metadata = SessionMetadata { ip_address, user_agent, device_type };

    match state.auth_app_service.sign_in(&params, &session_metadata).await {
        Ok(response) => Ok(HttpResponse::new(StatusCode::OK, "Sign in successfully ✅".to_string(), response)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
