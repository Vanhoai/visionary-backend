use axum::extract::State;
use std::sync::Arc;

// shared modules
use domain::entities::account_entity::AccountEntity;
use domain::usecases::account_usecases::ManageAccountsUseCase;

use crate::primary::middlewares::auth_middleware::AuthClaims;
// internal modules
use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    AuthClaims { account_id, .. }: AuthClaims,
) -> AxumResponse<Option<AccountEntity>> {
    match state.account_app_service.find_account_with_id(&account_id).await {
        Ok(account_option) => {
            if let Some(account_entity) = account_option {
                Ok(HttpResponse::new(
                    axum::http::StatusCode::OK,
                    "Account retrieved successfully ✅",
                    Some(account_entity),
                ))
            } else {
                Ok(HttpResponse::new(axum::http::StatusCode::NOT_FOUND, "Account not found 🙄", None))
            }
        },
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
