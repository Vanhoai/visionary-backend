use axum::http::StatusCode;

use domain::entities::account_entity::AccountEntity;
use domain::usecases::auth_usecases::{AuthParams, ManageSessionUseCases};

use crate::shared::di::container::DI_CONTAINER;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::utilities::validated_payload::ValidatedPayload;

pub async fn execute(
    ValidatedPayload(params): ValidatedPayload<AuthParams>,
) -> Result<HttpResponse<AccountEntity>, HttpFailure> {
    let auth_app_service = DI_CONTAINER.auth_app_service();

    match auth_app_service.sign_up(&params).await {
        Ok(response) => Ok(HttpResponse::new(StatusCode::OK, "Sign up successfully ✅".to_string(), response)),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
