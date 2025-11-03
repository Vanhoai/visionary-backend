use crate::shared::di::state::AppState;
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn execute() -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/sign-in", post(super::sign_in_function::execute))
        .route("/sign-up", post(super::sign_up_function::execute))
        .route("/oauth", post(super::oauth::execute));

    let private_routes = Router::new();

    Router::new().merge(public_routes).merge(private_routes)
}
