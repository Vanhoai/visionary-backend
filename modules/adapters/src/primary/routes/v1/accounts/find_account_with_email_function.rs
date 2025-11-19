use axum::extract::State;
use std::sync::Arc;

// shared modules
use domain::entities::account_entity::AccountEntity;
use domain::usecases::account_usecases::{FindAccountWithEmailQuery, ManageAccountsUseCase};

// internal modules
use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;
use crate::shared::utilities::validated_query::ValidatedQuery;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    ValidatedQuery(query): ValidatedQuery<FindAccountWithEmailQuery>,
) -> AxumResponse<Option<AccountEntity>> {
    match state.account_app_service.find_account_with_email(&query).await {
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
