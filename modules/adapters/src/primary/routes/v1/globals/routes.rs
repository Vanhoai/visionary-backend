use axum::routing::{delete, get, post, put};
use axum::{Router, middleware};
use std::sync::Arc;

// internal modules
use crate::primary::middlewares::auth_middleware::auth_middleware;
use crate::primary::middlewares::role_middleware::require_admin;
use crate::primary::routes::v1::globals::experiences::{
    add_experience_function, find_experiences_function, remove_experience_function, update_experience_function,
};
use crate::primary::routes::v1::globals::projects::{
    add_project_function, find_project_function, find_projects_function, remove_project_function,
    update_project_function,
};
use crate::shared::di::state::AppState;
use crate::shared::utilities::route_logger;

pub fn execute() -> Router<Arc<AppState>> {
    register_routes_for_logging();

    let protected_routes = Router::new()
        // experiences routes
        .route("/experiences", post(add_experience_function::execute))
        .route("/experiences/{id}", delete(remove_experience_function::execute))
        .route("/experiences/{id}", put(update_experience_function::execute))
        // projects routes
        .route("/projects", post(add_project_function::execute))
        .route("/projects/{id}", delete(remove_project_function::execute))
        .route("/projects/{id}", get(find_project_function::execute))
        .route("/projects/{id}", put(update_project_function::execute))
        .layer(middleware::from_fn(require_admin))
        .layer(middleware::from_fn(auth_middleware));

    let public_routes = Router::new()
        // experiences routes
        .route("/experiences", get(find_experiences_function::execute))
        // projects routes
        .route("/projects", get(find_projects_function::execute));

    protected_routes.merge(public_routes)
}

fn register_routes_for_logging() {
    route_logger::track_route("GET", "/api/v1/globals/experiences/", vec![]);
    route_logger::track_route("POST", "/api/v1/globals/experiences/", vec!["auth".to_string(), "admin".to_string()]);
    route_logger::track_route(
        "DELETE",
        "/api/v1/globals/experiences/{id}",
        vec!["auth".to_string(), "admin".to_string()],
    );
    route_logger::track_route("PUT", "/api/v1/globals/experiences/{id}", vec!["auth".to_string(), "admin".to_string()]);

    route_logger::track_route("GET", "/api/v1/globals/projects/", vec![]);
    route_logger::track_route("POST", "/api/v1/globals/projects/", vec!["auth".to_string(), "admin".to_string()]);
    route_logger::track_route("GET", "/api/v1/globals/projects/{id}", vec!["auth".to_string(), "admin".to_string()]);
    route_logger::track_route("DELETE", "/api/v1/globals/projects/{id}", vec!["auth".to_string(), "admin".to_string()]);
    route_logger::track_route("PUT", "/api/v1/globals/projects/{id}", vec!["auth".to_string(), "admin".to_string()]);
}
