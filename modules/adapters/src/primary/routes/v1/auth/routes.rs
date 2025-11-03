use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use crate::shared::di::state::AppState;

pub fn execute() -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/sign-in", post(super::sign_in::execute))
        .route("/sign-up", post(super::sign_up::execute))
        .route("/oauth", post(super::oauth::execute));

    let private_routes = Router::new();

    Router::new().merge(public_routes).merge(private_routes)
}
