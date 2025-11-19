use axum::extract::State;
use reqwest::StatusCode;
use std::sync::Arc;

// shared modules
use domain::usecases::auth_usecases::ManageSessionAuthUseCase;

// internal modules
use crate::shared::{
    di::state::AppState,
    models::{failure::HttpFailure, response::HttpResponse},
    types::AxumResponse,
};

pub async fn execute(State(state): State<Arc<AppState>>) -> AxumResponse<()> {
    match state.auth_app_service.sign_out().await {
        Ok(_) => Ok(HttpResponse::new(StatusCode::OK, "Signed out successfully ✅", ())),
        Err(failure) => Err(HttpFailure::new(failure)),
    }
}
