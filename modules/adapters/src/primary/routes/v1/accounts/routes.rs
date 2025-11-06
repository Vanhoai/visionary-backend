use crate::primary::middlewares::auth_middleware::auth_middleware;
use crate::shared::di::state::AppState;
use axum::Router;
use axum::middleware;
use axum::routing::{get, post, put};
use std::sync::Arc;

pub fn execute() -> Router<Arc<AppState>> {
    let public_routes = Router::new();
    let protected_routes = Router::new()
        .route("/", get(super::find_accounts_function::execute))
        .route("/{id}/experiences", get(super::experiences::find_experiences_function::execute))
        .route("/{id}/experiences", post(super::experiences::add_experience_function::execute))
        .route("/{id}/roles", post(super::roles::add_role_function::execute))
        .route("/{id}/roles", put(super::roles::update_role_function::execute))
        .layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}
