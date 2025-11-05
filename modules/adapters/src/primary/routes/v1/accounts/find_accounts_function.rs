use crate::shared::di::state::AppState;
use crate::shared::models::response::HttpPaginatedResponse;
use crate::shared::types::AxumPaginatedResponse;
use crate::shared::utilities::validated_query::ValidatedQuery;
use axum::extract::State;
use domain::entities::account_entity::AccountEntity;
use domain::usecases::account_usecases::{FindAccountsQuery, ManageAccountsUseCase};
use std::sync::Arc;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    ValidatedQuery(query): ValidatedQuery<FindAccountsQuery>,
) -> AxumPaginatedResponse<AccountEntity> {
    match state.account_app_service.find_accounts(&query).await {
        Ok((paginate, accounts)) => Ok(HttpPaginatedResponse::new(
            axum::http::StatusCode::OK,
            "Accounts retrieved successfully ✅".to_string(),
            paginate,
            accounts,
        )),
        Err(failure) => Err(crate::shared::models::failure::HttpFailure::new(failure)),
    }
}
