use axum::http::StatusCode;

use domain::usecases::auth_usecases::{AuthParams, AuthResponse, ManageSessionUseCases};

use crate::shared::di::container::instance;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::utilities::validated_payload::ValidatedPayload;

pub async fn execute(
    ValidatedPayload(params): ValidatedPayload<AuthParams>,
) -> Result<HttpResponse<AuthResponse>, HttpFailure> {
    let auth_app_service = instance().await.auth_app_service();

    match auth_app_service.sign_in(&params).await {
        Ok(response) => Ok(HttpResponse::new(StatusCode::OK, "Sign in successfully ✅".to_string(), response)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
