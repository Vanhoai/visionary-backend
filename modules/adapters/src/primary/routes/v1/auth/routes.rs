use crate::shared::di::state::AppState;
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn execute() -> Router<Arc<AppState>> {
    let public_routes = Router::new()
        .route("/sign-in", post(super::sign_in_function::execute))
        .route("/sign-up", post(super::sign_up_function::execute))
        .route("/oauth2/init", get(super::oauth_init_function::execute))
        .route("/oauth2/callback", get(super::oauth_callback_function::execute));

    let private_routes = Router::new();

    Router::new().merge(public_routes).merge(private_routes)
}
