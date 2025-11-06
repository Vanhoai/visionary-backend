use crate::shared::di::state::AppState;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn execute() -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/{id}/experiences", get(super::experiences::find_experiences_function::execute))
        .route("/{id}/experiences", post(super::experiences::add_experience_function::execute))
        .route("/", get(super::find_accounts_function::execute));

    let private_routes = Router::new();

    Router::new().merge(public_routes).merge(private_routes)
}
