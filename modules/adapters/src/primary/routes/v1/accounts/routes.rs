use axum::Router;

pub fn execute() -> Router {
    let public_routes = Router::new();
    let private_routes = Router::new();

    Router::new().merge(public_routes).merge(private_routes)
}
