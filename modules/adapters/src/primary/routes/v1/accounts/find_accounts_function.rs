use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::entities::account_entity::AccountEntity;
use domain::usecases::account_usecases::{FindAccountsQuery, ManageAccountsUseCase};

// internal modules
use crate::primary::middlewares::auth_middleware::AuthClaims;
use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpPaginatedResponse;
use crate::shared::types::AxumPaginatedResponse;
use crate::shared::utilities::validated_query::ValidatedQuery;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    ValidatedQuery(query): ValidatedQuery<FindAccountsQuery>,
    AuthClaims { .. }: AuthClaims,
) -> AxumPaginatedResponse<AccountEntity> {
    match state.account_app_service.find_accounts(&query).await {
        Ok((paginate, accounts)) => Ok(HttpPaginatedResponse::new(
            StatusCode::OK,
            "Accounts retrieved successfully ✅".to_string(),
            paginate,
            accounts,
        )),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
