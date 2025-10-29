use axum::Router;
use axum::routing::post;

pub fn execute() -> Router {
    let public_routes = Router::new()
        .route("/sign-in", post(super::sign_in::execute))
        .route("/sign-up", post(super::sign_up::execute))
        .route("/oauth", post(super::oauth::execute));

    let private_routes = Router::new();

    Router::new().merge(public_routes).merge(private_routes)
}
