use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

// internal modules
use crate::primary::routes::v1::auth::oauth_callback_function;
use crate::primary::routes::v1::auth::oauth_init_function;
use crate::primary::routes::v1::auth::refresh_token_function;
use crate::primary::routes::v1::auth::sign_in_function;
use crate::primary::routes::v1::auth::sign_out_function;
use crate::primary::routes::v1::auth::sign_up_function;
use crate::shared::di::state::AppState;
use crate::shared::utilities::route_logger;

pub fn execute() -> Router<Arc<AppState>> {
    register_routes_for_logging();
    let public_routes = Router::new()
        .route("/sign-in", post(sign_in_function::execute))
        .route("/sign-up", post(sign_up_function::execute))
        .route("/refresh-token", post(refresh_token_function::execute))
        .route("/sign-out", post(sign_out_function::execute))
        .route("/oauth2/init", get(oauth_init_function::execute))
        .route("/oauth2/callback", get(oauth_callback_function::execute));

    let private_routes = Router::new();
    Router::new().merge(public_routes).merge(private_routes)
}

fn register_routes_for_logging() {
    route_logger::track_route("POST", "/api/v1/auth/sign-in", vec![]);
    route_logger::track_route("POST", "/api/v1/auth/sign-up", vec![]);
    route_logger::track_route("POST", "/api/v1/auth/refresh-token", vec![]);
    route_logger::track_route("POST", "/api/v1/auth/sign-out", vec![]);
    route_logger::track_route("GET", "/api/v1/auth/oauth2/init", vec![]);
    route_logger::track_route("GET", "/api/v1/auth/oauth2/callback", vec![]);
}
