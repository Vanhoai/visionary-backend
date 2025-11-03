use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::utilities::validated_payload::ValidatedPayload;
use axum::extract::State;
use axum::http::StatusCode;
use domain::usecases::auth_usecases::{AuthParams, AuthResponse, ManageSessionUseCases};
use std::sync::Arc;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    ValidatedPayload(params): ValidatedPayload<AuthParams>,
) -> Result<HttpResponse<AuthResponse>, HttpFailure> {
    match state.auth_app_service.sign_in(&params).await {
        Ok(response) => Ok(HttpResponse::new(StatusCode::OK, "Sign in successfully ✅".to_string(), response)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
