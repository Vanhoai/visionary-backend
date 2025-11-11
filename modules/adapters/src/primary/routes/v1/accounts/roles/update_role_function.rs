use axum::extract::{Path, State};
use axum::http::StatusCode;
use std::sync::Arc;

// shared modules
use domain::entities::role_entity::RoleEntity;
use domain::usecases::account_usecases::{ManageRoleAccountUseCase, UpdateRoleToAccountParams};

// internal modules
use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;
use crate::shared::utilities::validated_payload::ValidatedPayload;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ValidatedPayload(params): ValidatedPayload<UpdateRoleToAccountParams>,
) -> AxumResponse<RoleEntity> {
    match state.account_app_service.update_role_for_account(&id, &params).await {
        Ok(role_entity) => Ok(HttpResponse::new(StatusCode::OK, "Role updated successfully", role_entity)),
        Err(e) => Err(HttpFailure::new(e)),
    }
}
