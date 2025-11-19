use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

// internal modules
use crate::primary::routes::v1::globals::experiences::{add_experience_function, find_experiences_function};
use crate::shared::di::state::AppState;
use crate::shared::utilities::route_logger;

pub fn execute() -> Router<Arc<AppState>> {
    register_routes_for_logging();

    let experiences = Router::new()
        .route("/", post(add_experience_function::execute))
        .route("/", get(find_experiences_function::execute));

    Router::new().nest("/experiences", experiences)
}

fn register_routes_for_logging() {
    route_logger::track_route("GET", "/api/v1/globals/experiences/", vec![]);
    route_logger::track_route("POST", "/api/v1/globals/experiences/", vec![]);
}
