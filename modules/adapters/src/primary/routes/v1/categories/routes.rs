use axum::routing::{get, post, put};
use axum::{Router, middleware};
use std::sync::Arc;

// internal modules
use crate::primary::middlewares::auth_middleware::auth_middleware;
use crate::primary::middlewares::role_middleware::require_admin;
use crate::shared::di::state::AppState;

pub fn execute() -> Router<Arc<AppState>> {
    let public_routes = Router::new();
    let protected_routes = Router::new()
        .route("/", get(super::find_categories_function::execute))
        .route("/", post(super::create_category_function::execute))
        .route("/{id}", put(super::update_category_function::execute))
        .layer(middleware::from_fn(require_admin))
        .layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}
