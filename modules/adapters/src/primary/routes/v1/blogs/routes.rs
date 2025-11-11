use axum::{Router, routing::get};
use std::sync::Arc;

// internal modules
use crate::shared::di::state::AppState;

pub fn execute() -> Router<Arc<AppState>> {
    let public_routes = Router::new().route("/", get(super::find_blogs_function::execute));
    let protected_routes = Router::new();

    Router::new().merge(public_routes).merge(protected_routes)
}
