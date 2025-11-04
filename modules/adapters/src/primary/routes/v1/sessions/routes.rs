use crate::shared::di::state::AppState;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn execute() -> Router<Arc<AppState>> {
    let public_routes = Router::new().route("/", get(super::find_sessions_function::execute));
    let private_routes = Router::new();

    Router::new().merge(public_routes).merge(private_routes)
}
