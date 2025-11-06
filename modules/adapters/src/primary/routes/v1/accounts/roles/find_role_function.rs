use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;
use axum::extract::{Path, State};
use domain::entities::role_entity::RoleEntity;
use domain::usecases::account_usecases::ManageRoleAccountUseCase;
use reqwest::StatusCode;
use std::sync::Arc;

pub async fn execute(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> AxumResponse<RoleEntity> {
    match state.account_app_service.find_role_by_account_id(&id).await {
        Ok(role) => Ok(HttpResponse::new(StatusCode::FOUND, "Retrieved role successfully", role)),
        Err(err) => Err(HttpFailure::new(err)),
    }
}
