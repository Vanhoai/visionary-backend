use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

use domain::entities::account_entity::AccountEntity;
use domain::usecases::auth_usecases::{AuthParams, ManageSessionAuthUseCase};

use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;
use crate::shared::utilities::validated_payload::ValidatedPayload;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    ValidatedPayload(params): ValidatedPayload<AuthParams>,
) -> AxumResponse<AccountEntity> {
    match state.auth_app_service.sign_up(&params).await {
        Ok(response) => Ok(HttpResponse::new(StatusCode::OK, "Sign up successfully ✅".to_string(), response)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
