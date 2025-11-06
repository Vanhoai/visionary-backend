use crate::shared::di::state::AppState;
use crate::shared::models::failure::HttpFailure;
use crate::shared::models::response::HttpResponse;
use crate::shared::types::AxumResponse;
use crate::shared::utilities::validated_payload::ValidatedPayload;
use axum::extract::{Path, State};
use domain::entities::role_entity::RoleEntity;
use domain::usecases::account_usecases::{AddRoleToAccountParams, ManageRoleAccountUseCase};
use reqwest::StatusCode;
use std::sync::Arc;

pub async fn execute(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ValidatedPayload(params): ValidatedPayload<AddRoleToAccountParams>,
) -> AxumResponse<RoleEntity> {
    match state.account_app_service.add_role_to_account(&id, &params).await {
        Ok(role_entity) => {
            Ok(HttpResponse::new(StatusCode::CREATED, "Role added to account successfully", role_entity))
        },
        Err(e) => Err(HttpFailure::new(e)),
    }
}
