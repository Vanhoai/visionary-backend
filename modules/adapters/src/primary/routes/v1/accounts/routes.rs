use std::sync::Arc;
use axum::Router;
use crate::shared::di::state::AppState;

pub fn execute() -> Router<Arc<AppState>> {
    let public_routes = Router::new();
    let private_routes = Router::new();

    Router::new().merge(public_routes).merge(private_routes)
}
