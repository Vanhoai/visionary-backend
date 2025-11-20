use axum::routing::{delete, get, post};
use axum::{Router, middleware};
use std::sync::Arc;

use crate::primary::middlewares::auth_middleware::auth_middleware;
use crate::primary::middlewares::role_middleware::require_admin;
// internal modules
use crate::primary::routes::v1::globals::experiences::{add_experience_function, find_experiences_function};
use crate::primary::routes::v1::globals::projects::{
    add_project_function, find_project_function, find_projects_function, remove_project_function,
};
use crate::shared::di::state::AppState;
use crate::shared::utilities::route_logger;

pub fn execute() -> Router<Arc<AppState>> {
    register_routes_for_logging();

    let protected_routes = Router::new()
        .route("/experiences", post(add_experience_function::execute))
        .route("/projects", post(add_project_function::execute))
        .route("/projects/{id}", delete(remove_project_function::execute))
        .route("/projects/{id}", get(find_project_function::execute))
        .layer(middleware::from_fn(require_admin))
        .layer(middleware::from_fn(auth_middleware));

    let public_routes = Router::new()
        .route("/experiences", get(find_experiences_function::execute))
        .route("/projects", get(find_projects_function::execute));

    protected_routes.merge(public_routes)
}

fn register_routes_for_logging() {
    route_logger::track_route("GET", "/api/v1/globals/experiences/", vec![]);
    route_logger::track_route("POST", "/api/v1/globals/experiences/", vec!["auth".to_string(), "admin".to_string()]);

    route_logger::track_route("GET", "/api/v1/globals/projects/", vec![]);
    route_logger::track_route("POST", "/api/v1/globals/projects/", vec!["auth".to_string(), "admin".to_string()]);
    route_logger::track_route("DELETE", "/api/v1/globals/projects/{id}", vec!["auth".to_string(), "admin".to_string()]);
    route_logger::track_route("GET", "/api/v1/globals/projects/{id}", vec!["auth".to_string(), "admin".to_string()]);
}
