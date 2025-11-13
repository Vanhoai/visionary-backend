use axum::Router;
use axum::middleware;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

// internal modules
use crate::primary::middlewares::auth_middleware::auth_middleware;
use crate::primary::middlewares::role_middleware::{require_admin, require_roles};
use crate::primary::routes::v1::accounts::blogs::delete_blog_function;
use crate::primary::routes::v1::accounts::blogs::publish_blog_function;
use crate::primary::routes::v1::accounts::blogs::update_blog_function;
use crate::primary::routes::v1::accounts::experiences::add_experience_function;
use crate::primary::routes::v1::accounts::experiences::find_experiences_function;
use crate::primary::routes::v1::accounts::find_accounts_function;
use crate::primary::routes::v1::accounts::roles::add_role_function;
use crate::primary::routes::v1::accounts::roles::find_role_function;
use crate::primary::routes::v1::accounts::roles::update_role_function;
use crate::shared::di::state::AppState;

pub fn execute() -> Router<Arc<AppState>> {
    let public_routes = Router::new();

    let protected_routes = Router::new()
        .route("/", get(find_accounts_function::execute).layer(middleware::from_fn(require_admin)))
        .route(
            "/{id}/experiences",
            get(find_experiences_function::execute).layer(middleware::from_fn(require_roles(vec![]))),
        )
        .route("/{id}/experiences", post(add_experience_function::execute))
        .route("/{id}/roles", post(add_role_function::execute))
        .route("/{id}/roles", put(update_role_function::execute))
        .route("/{id}/roles", get(find_role_function::execute))
        .route("/{id}/blogs", post(publish_blog_function::execute))
        .route("/{account_id}/blogs/{blog_id}", put(update_blog_function::execute))
        .route("/{account_id}/blogs/{blog_id}", delete(delete_blog_function::execute))
        .layer(middleware::from_fn(auth_middleware));

    Router::new().merge(public_routes).merge(protected_routes)
}
