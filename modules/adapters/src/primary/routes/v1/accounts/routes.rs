use axum::Router;
use axum::middleware;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

// internal modules
use crate::primary::middlewares::auth_middleware::auth_middleware;
use crate::primary::middlewares::role_middleware::require_admin;
use crate::primary::routes::v1::accounts::blogs::delete_blog_function;
use crate::primary::routes::v1::accounts::blogs::publish_blog_function;
use crate::primary::routes::v1::accounts::blogs::update_blog_function;
use crate::primary::routes::v1::accounts::roles::add_role_function;
use crate::primary::routes::v1::accounts::roles::find_role_function;
use crate::primary::routes::v1::accounts::roles::update_role_function;
use crate::primary::routes::v1::accounts::{
    find_account_with_email_function, find_accounts_function, find_profile_function,
};
use crate::shared::di::state::AppState;
use crate::shared::utilities::route_logger;

pub fn execute() -> Router<Arc<AppState>> {
    register_routes_for_logging();
    let public_routes = Router::new().route("/find-account-with-email", get(find_account_with_email_function::execute));

    let protected_routes = Router::new()
        .route("/", get(find_accounts_function::execute).layer(middleware::from_fn(require_admin)))
        .route("/{id}/roles", post(add_role_function::execute))
        .route("/{id}/roles", put(update_role_function::execute))
        .route("/{id}/roles", get(find_role_function::execute))
        .route("/{id}/blogs", post(publish_blog_function::execute))
        .route("/{account_id}/blogs/{blog_id}", put(update_blog_function::execute))
        .route("/{account_id}/blogs/{blog_id}", delete(delete_blog_function::execute))
        .route("/find-profile", get(find_profile_function::execute))
        .layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}

fn register_routes_for_logging() {
    route_logger::track_route("GET", "/api/v1/accounts/find-account-with-email", vec![]);

    route_logger::track_route("GET", "/api/v1/accounts/", vec!["admin".to_string()]);
    route_logger::track_route("POST", "/api/v1/accounts/{id}/roles", vec![]);
    route_logger::track_route("PUT", "/api/v1/accounts/{id}/roles", vec![]);
    route_logger::track_route("GET", "/api/v1/accounts/{id}/roles", vec![]);
    route_logger::track_route("POST", "/api/v1/accounts/{id}/blogs", vec![]);
    route_logger::track_route("PUT", "/api/v1/accounts/{account_id}/blogs/{blog_id}", vec![]);
    route_logger::track_route("DELETE", "/api/v1/accounts/{account_id}/blogs/{blog_id}", vec![]);
    route_logger::track_route("GET", "/api/v1/accounts/find-profile", vec!["auth".to_string()]);
}
